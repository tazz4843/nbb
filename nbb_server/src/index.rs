use crate::errors::WebServerError;
use axum::http::{HeaderMap, HeaderValue};
use nbb_renderer::{render_index_page, BlogPost};
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::time::UNIX_EPOCH;

pub async fn index() -> Result<(HeaderMap, String), WebServerError> {
    let cfg = nbb_config::get_config();
    let mut res = Vec::new();
    let mut read_dir = tokio::fs::read_dir(&cfg.general.data_dir).await?;
    while let Some(entry) = read_dir.next_entry().await? {
        let path = entry.path();
        if path.extension() == Some(OsStr::from_bytes(b"md")) {
            let filename = path
                .file_stem()
                .expect("extension should already exist")
                .to_string_lossy()
                .to_string();
            let metadata = tokio::fs::metadata(&path).await?;
            let created_at = metadata
                .created()
                .or_else(|_| metadata.modified())?
                .duration_since(UNIX_EPOCH)?
                .as_secs();
            let target = format!("/blog/{}", &filename).into();

            res.push(BlogPost::post_index(
                filename.into(),
                None,
                created_at,
                target,
            ));
        }
    }

    let rendered_html = tokio::task::spawn_blocking(move || {
        let blog_posts = res;
        render_index_page(&blog_posts[..])
    })
    .await??;

    let mut headers = HeaderMap::with_capacity(1);
    headers.insert("Content-Type", HeaderValue::from_static("text/html"));
    Ok((headers, rendered_html))
}
