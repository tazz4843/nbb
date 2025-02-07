use crate::post::BlogPost;
use tera::{Context, Error};

pub const BLOG_POST_TEMPLATE_FILENAME: &str = "blog_post.html";

/// Render a singular blog post.
///
/// `post` should be the post to be rendered.
///
/// # Errors
/// Propagates any errors thrown by Tera.
#[inline]
pub fn render_blog_post(post: &BlogPost) -> Result<String, Error> {
    let mut ctx = Context::default();
    ctx.insert("post", post);

    crate::renderer::render(BLOG_POST_TEMPLATE_FILENAME, ctx)
}
