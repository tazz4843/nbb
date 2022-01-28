use crate::errors::WebServerError;
use axum::extract::Path as WebPath;
use axum::http::{HeaderMap, HeaderValue};

pub async fn blog_post_assets(
    WebPath((path, file)): WebPath<(String, String)>,
) -> Result<(HeaderMap, Vec<u8>), WebServerError> {
    let cfg = nbb_config::get_config();
    let mut target = cfg.general.data_dir.join(&path);
    target.push(file);
    debug!(
        request_path=%path,
        resolved_path=?target,
        "got request for static data of post at {}, resolved path to {:?}",
        path, &target
    );

    if !target.exists() {
        debug!(request_path=%path, resolved_path=?target, "path not found");
        return Err(WebServerError::NotFound);
    }

    let data = tokio::fs::read(&target).await?;
    let mut headers = HeaderMap::with_capacity(1);
    if let Some(mime) = mime_guess::MimeGuess::from_path(&target).first_raw() {
        headers.insert("Content-Type", HeaderValue::from_static(mime));
    }
    Ok((headers, data))
}
