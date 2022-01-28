use crate::render_markdown;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::time::Instant;

pub fn prerender_all(path: &Path) {
    let dir_list = std::fs::read_dir(path).expect("failed to list blog dir");
    debug!(
        "pre-rendering an estimated {} posts",
        dir_list.size_hint().0
    );
    let st = Instant::now();
    let mut i = 0;
    for file in dir_list {
        let file = file.expect("IO error while listing directory");
        let path = file.path();
        if !path.is_file() || path.extension() != Some(OsStr::from_bytes(b"md")) {
            continue;
        }
        debug!("rendering {:?}", path);
        std::mem::drop(render_markdown(&path));
        i += 1;
    }
    debug!(
        "pre-rendered {} posts in {}ms",
        i,
        (st.elapsed().as_nanos() as f64) / 1_000_000.0
    );
}
