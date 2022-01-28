#![feature(once_cell)]

mod cache;
mod cfg;
mod prerender;
mod render;

pub use prerender::prerender_all;
pub use render::render_markdown;
