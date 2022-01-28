use crate::errors::WebServerError;
use axum::extract::Path as WebPath;
use axum::http::{HeaderMap, HeaderValue};
use nbb_renderer::render_blog_post;
use std::time::UNIX_EPOCH;

pub async fn blog_post(
    WebPath(path): WebPath<String>,
) -> Result<(HeaderMap, String), WebServerError> {
    let cfg = nbb_config::get_config();
    let mut target = cfg.general.data_dir.join(&path);
    target.set_extension("md");
    debug!(
        request_path=%path,
        resolved_path=?target,
        "got request for blog post at {}, resolved path to {:?}",
        path, &target
    );

    if !target.exists() {
        debug!(request_path=%path, resolved_path=?target, "path not found");
        return Err(WebServerError::NotFound);
    }

    debug!(request_path=%path, resolved_path=?target, "fetching/rendering Markdown");
    let (target, rendered_md) = tokio::task::spawn_blocking(move || {
        let target = target;
        let rendered_md = nbb_markdown::render_markdown(&target);
        (target, rendered_md)
    })
    .await?;

    debug!(request_path=%path, resolved_path=?target, "getting created_at");
    let metadata = tokio::fs::metadata(&target).await?;
    let created_at = metadata
        .created()
        .or_else(|_| metadata.modified())?
        .duration_since(UNIX_EPOCH)?
        .as_secs();

    debug!(request_path=%path, resolved_path=?target, "rendering HTML page");
    let blog_post =
        nbb_renderer::BlogPost::post_page(path.into(), None, created_at, rendered_md.into());
    let rendered_html = tokio::task::spawn_blocking(move || render_blog_post(&blog_post)).await??;

    let mut headers = HeaderMap::with_capacity(1);
    headers.insert("Content-Type", HeaderValue::from_static("text/html"));
    Ok((headers, rendered_html))
}
