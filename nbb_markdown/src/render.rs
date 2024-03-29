use std::convert::identity;
use std::path::Path;

/// Render Markdown to HTML.
///
/// This function also handles caching of rendered HTML.
///
/// # Panics
/// This function panics if the requested file does not exist.
/// Be sure to check it exists before calling this function.
#[must_use]
pub fn render_markdown(path: &Path) -> String {
    let cfg = crate::cfg::get_config();

    crate::cache::get_from_cache(path).map_or_else(
        || {
            let md = std::fs::read_to_string(path).expect("file doesn't exist!");
            let rendered = comrak::markdown_to_html(&md, cfg);
            crate::cache::insert_into_cache(path.to_path_buf(), rendered.clone());
            rendered
        },
        identity,
    )
}
