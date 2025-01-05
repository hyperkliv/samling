# NOTE: We use the custom allocator https://github.com/microsoft/mimalloc because
#       of performance issues with the default musl allocator.
#       Build instructions found here:
#       https://github.com/emerzon/alpine-mimalloc/blob/master/Dockerfile
# NOTE: This has to be activated from within the Rust code,
#       e.g. with https://crates.io/crates/mimalloc.
ARG API_BASE_IMAGE=lukemathwalker/cargo-chef:0.1.62-rust-1.75.0-alpine3.19


### Mimalloc compilation
FROM $API_BASE_IMAGE as mimalloc_builder
WORKDIR /
ARG MIMALLOC_VERSION=2.1.2
RUN apk add --no-cache build-base cmake linux-headers \
    && wget -O- https://github.com/microsoft/mimalloc/archive/refs/tags/v$MIMALLOC_VERSION.tar.gz | tar xzf - \
    && cmake mimalloc-$MIMALLOC_VERSION \
    && make -j$(nproc) \
    && make install


### UI builder
FROM node:20.6.0-alpine3.18 as ui_builder
WORKDIR /ui
COPY ./ui/ ./
RUN npm ci

# NOTE: We default to `REACT_APP_API_BASE_URL=/api` to avoid CORS configuration. This
#       also means that we can use the same static build for both our staging and
#       production environments.
ARG REACT_APP_API_BASE_URL=/api

RUN npm run build


### Cache planner for Rust dependencies
FROM $API_BASE_IMAGE as api_planner
WORKDIR /app

COPY /.cargo/ ./.cargo/
COPY /src/ ./src/
COPY /migrations/ ./migrations/
COPY Cargo.toml Cargo.lock ./
RUN cargo chef prepare --recipe-path recipe.json


### Build Rust project
FROM $API_BASE_IMAGE as api_builder
WORKDIR /app

# Add dependencies needed for ssh2, or specifically the vendored openssl build
RUN apk add --no-cache musl-dev openssl-dev perl make

COPY --from=mimalloc_builder /libmimalloc.so.2 /lib
ENV LD_PRELOAD=/lib/libmimalloc.so.2 MIMALLOC_LARGE_OS_PAGES=1

COPY --from=api_planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY --from=ui_builder /ui/build/ ./ui/build/

COPY /src/ ./src/
COPY /migrations/ ./migrations/
COPY Cargo.toml Cargo.lock ./

RUN cargo build \
    --features ui \
    --features mimalloc \
    --release


### The resulting app image
FROM scratch as runtime
WORKDIR /usr/local/bin
ENV APP_HOST=0.0.0.0
COPY --from=api_builder /app/target/release/samling .
ENTRYPOINT [ "/usr/local/bin/samling" ]
CMD [ "serve" ]
