use axum::handler::HandlerWithoutStateExt;
use axum::routing::{get_service, MethodRouter};
use tower_http::services::ServeDir;

pub fn get_static_service() -> MethodRouter {
    async fn not_found_svc() -> &'static str {
        "404 Not Found"
    }

    let asset_path = nbb_config::get_config()
        .general
        .data_dir
        .join("static")
        .canonicalize()
        .expect("failed to canonicalize static asset path");
    debug!("static asset path: {}", asset_path.display());

    get_service(ServeDir::new(asset_path).not_found_service(not_found_svc.into_service()))
}
