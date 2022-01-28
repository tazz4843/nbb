#![feature(once_cell)]

#[macro_use]
extern crate tracing;

mod cache;
mod cfg;
mod prerender;
mod render;

pub use prerender::prerender_all;
pub use render::render_markdown;
