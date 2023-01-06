use crate::blog_post::blog_post;
use crate::blog_post_assets::blog_post_assets;
use crate::index::index;
use crate::info::info;
use crate::not_found::not_found;
use axum::http::StatusCode;
use axum::routing::{get, get_service};
use axum::Router;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

pub fn build_router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/blog/:title", get(blog_post))
        .route("/blog/:title/:file", get(blog_post_assets))
        .route(
            "/static/:file",
            get_service(ServeDir::new("./static")).handle_error(
                |error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                },
            ),
        )
        .route("/info", get(info))
        .fallback(not_found)
        .layer(TraceLayer::new_for_http())
}
