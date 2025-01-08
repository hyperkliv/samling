use std::net::SocketAddr;

use axum::extract::{DefaultBodyLimit, State};
use axum::http::Method;
use axum::Json;
use http::header::{AUTHORIZATION, CONTENT_DISPOSITION, CONTENT_TYPE};
use http::HeaderValue;
use tokio::net::TcpListener;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::auth::hashing::Hasher;
use crate::auth::signing::JwtSigner;
use crate::cloudflare::CloudflareApi;
use crate::db_migrations::Migrator;
use crate::deadpool_postgres::Pool;
use crate::errors::{CliResult, Result};
use crate::routes::AppRouter;
use crate::state::AppState;
use crate::{
    admin, attributes, auth, categories, collections, colors, exports, images, organizations,
    prices, sizes, styles, Environment, Error,
};

pub async fn serve(
    address: SocketAddr,
    db_pool: Pool,
    secret: String,
    cors_allowed_origins: Vec<HeaderValue>,
    cloudflare_api: CloudflareApi,
    environment: Environment,
) -> CliResult<()> {
    let hasher = Hasher::default();
    let jwt_signer = JwtSigner::new(&secret);

    let app_state = AppState::new(environment, db_pool, cloudflare_api, jwt_signer, hasher);

    // TODO: Don't hard-code origins
    let cors_layer = CorsLayer::new()
        .allow_origin(cors_allowed_origins)
        .allow_headers([CONTENT_TYPE, AUTHORIZATION, CONTENT_DISPOSITION])
        .allow_methods([
            Method::HEAD,
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
        ]);

    let org_router = AppRouter::new().nest(
        "/{organization_id}",
        AppRouter::new()
            .merge(auth::routes::org_routes())
            .merge(collections::routes::org_routes())
            .merge(attributes::routes::org_routes())
            .merge(styles::routes::org_routes())
            .merge(colors::routes::org_routes())
            .merge(sizes::routes::org_routes())
            .merge(categories::routes::org_routes())
            .merge(images::routes::org_routes())
            .merge(prices::routes::org_routes())
            .merge(exports::routes::org_routes())
            .merge(admin::routes::org_routes()),
    );

    // NOTE: Disabling brotli compression due to very slow HTTP responses (10x slower and more)
    //       See these links for more info:
    //       - https://github.com/tower-rs/tower-http/pull/356
    //       - https://github.com/dropbox/rust-brotli/issues/93
    let compression_layer = CompressionLayer::new().br(false);

    let api_router = AppRouter::new()
        .merge(auth::routes::global_routes())
        .merge(organizations::routes::global_routes())
        .merge(org_router)
        .route("/livez", axum::routing::get(livez))
        .route("/readyz", axum::routing::get(readyz))
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer)
        .layer(compression_layer);

    let app = make_app(api_router)
        .with_state(app_state)
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024));

    tracing::info!("Listening on {}", address);
    let listener = TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[cfg(feature = "ui")]
fn make_app(api_router: AppRouter) -> AppRouter {
    use crate::ui;

    AppRouter::new()
        .nest("/api", api_router)
        .fallback(ui::serve_spa)
}

#[cfg(not(feature = "ui"))]
fn make_app(api_router: AppRouter) -> AppRouter {
    AppRouter::new().nest("/api", api_router)
}

async fn livez() -> Json<serde_json::Value> {
    Json(serde_json::json!({"ok": true}))
}

async fn readyz(State(migrator): State<Migrator>) -> Result<Json<serde_json::Value>> {
    let summary = migrator.summary().await?;
    if summary.up_to_date() {
        Ok(Json(serde_json::json!({"ok": true})))
    } else {
        Err(Error::ApplicationNotReady)
    }
}
