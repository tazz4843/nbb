use crate::post::BlogPost;
use tera::{Context, Error};

/// Render the index page.
///
/// `posts` should be a Vec of blog posts.
///
/// # Errors
/// Propagates any errors thrown by Tera.
pub fn render_index_page(posts: &[BlogPost]) -> Result<String, Error> {
    let mut ctx = Context::default();
    ctx.insert("posts", &posts);

    crate::renderer::render("index", ctx)
}
