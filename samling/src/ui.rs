use axum::http::{status::StatusCode, HeaderMap, HeaderValue};
use include_dir::{include_dir, Dir};

static FILES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../ui/build");

/// Serve our create-react-app UI
///
/// Respond with static files or index.html if not found
///
/// The idea is to use this with Router::fallback
pub async fn serve_spa(uri: axum::http::Uri) -> (StatusCode, HeaderMap, &'static [u8]) {
    let mut header_map = HeaderMap::new();
    let uri_path = match uri.path() {
        "/" => "index.html",
        other => other.trim_start_matches('/'),
    };

    if uri_path.starts_with("api/") || uri_path == "api" {
        return (StatusCode::NOT_FOUND, header_map, b"Not Found");
    }

    let file = match FILES.get_file(uri_path) {
        Some(file) => Some(file),
        None => FILES.get_file("index.html"),
    };
    if let Some(file) = file {
        let path = file.path();
        let guess = mime_guess::from_path(path);
        let mime = guess
            .first_raw()
            .map(HeaderValue::from_static)
            .unwrap_or_else(|| {
                HeaderValue::from_str(mime::APPLICATION_OCTET_STREAM.as_ref()).unwrap()
            });
        header_map.insert("Content-Type", mime);
        (StatusCode::OK, header_map, file.contents())
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            header_map,
            b"Internal server error - UI probably not built",
        )
    }
}
