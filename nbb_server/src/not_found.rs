use axum::http::{HeaderMap, HeaderValue};

#[allow(clippy::unused_async)]
pub async fn not_found() -> (HeaderMap, String) {
    let mut headers = HeaderMap::with_capacity(1);
    headers.insert("Content-Type", HeaderValue::from_static("text/html"));
    (headers, nbb_renderer::render_404())
}
