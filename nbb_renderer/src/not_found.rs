use tera::Context;

/// Render the 404 not found page.
///
/// # Panics
/// Panics if Tera throws an error.
#[inline]
#[must_use]
pub fn render_404() -> String {
    let ctx = Context::default();

    crate::renderer::render("404", ctx).expect("failed to render 404 page")
}
