#[allow(clippy::unused_async)]
pub async fn not_found() -> String {
    nbb_renderer::render_404()
}
