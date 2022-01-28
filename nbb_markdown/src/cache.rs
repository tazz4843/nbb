use blake2::{Blake2b512, Digest};
use dashmap::DashMap;
use once_cell::sync::OnceCell;
use std::io::Read;
use std::path::{Path, PathBuf};

static DISK_FILE_HASHES: OnceCell<DashMap<PathBuf, Vec<u8>>> = OnceCell::new();
static RENDERER_CACHE: OnceCell<DashMap<PathBuf, String>> = OnceCell::new();

/// Check if a file on disk matches what is cached.
///
/// Returns false if the file on disk does NOT match the cache, or if an error happened in any case.
fn check_hash(path: &Path) -> bool {
    let mut f = match std::fs::OpenOptions::new().read(true).open(&path) {
        Ok(f) => f,
        Err(_) => return false,
    };

    let mut buf = vec![0; 64 * 1024]; // read 64kb at a time
    let mut eof = false;
    let mut hasher = Blake2b512::new();
    loop {
        match f.read(&mut buf) {
            Ok(b) if b < 64 * 1024 => eof = true,
            Err(_) => return false,
            _ => {}
        }
        hasher.update(&mut buf);
        if eof {
            break;
        }
        // overwrite the buffer
        buf = vec![0; 64 * 1024];
    }

    let hash = hasher.finalize().into_iter().collect::<Vec<_>>();

    let disk_file_hashes = DISK_FILE_HASHES.get_or_init(DashMap::new);
    if let Some(x) = disk_file_hashes.get(path) {
        x.value() == &hash
    } else {
        disk_file_hashes.insert(path.to_path_buf(), hash);
        false
    }
}

/// Get a rendered object from cache, or return None if this needs to be re-rendered/was not found.
pub fn get_from_cache(path: &Path) -> Option<String> {
    if !check_hash(path) {
        return None;
    }
    RENDERER_CACHE
        .get_or_init(DashMap::new)
        .get(path)
        .map(|x| x.value().clone())
}

/// Insert a freshly-rendered object into cache.
///
/// Returns true if this overwrote an existing object.
pub fn insert_into_cache(path: PathBuf, html: String) -> bool {
    RENDERER_CACHE
        .get_or_init(DashMap::new)
        .insert(path, html)
        .map_or(false, |_| true)
}
