use std::fs;
use std::path::Path;
use std::env;

pub fn read_to_string<P: AsRef<Path>>(path: P) -> Option<String> {
    let path = path.as_ref();
    fs::read_to_string(env::current_exe().ok()?.parent()?.join(path)).ok()
        .or_else(|| fs::read_to_string(env::current_dir().ok()?.join(path)).ok())
        .or_else(|| fs::read_to_string(env::current_dir().ok()?.canonicalize().ok()?.parent()?.join(path)).ok())
}
