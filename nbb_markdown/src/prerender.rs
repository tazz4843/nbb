use crate::render_markdown;
use std::path::Path;

pub fn prerender_all(path: &Path) {
    for file in std::fs::read_dir(path).expect("failed to list blog dir") {
        let file = file.expect("IO error while listing directory");
        std::mem::drop(render_markdown(&file.path()));
    }
}
