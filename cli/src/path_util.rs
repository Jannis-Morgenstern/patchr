use std::path::{Path, PathBuf};

pub fn find_file_in_parent_dirs(starting_directory: &Path, file_name: &str) -> Option<PathBuf> {
    let mut path: PathBuf = starting_directory.into();
    let file = Path::new(file_name);

    loop {
        path.push(file);
        if path.is_file() {
            break Some(path);
        }
        if !(path.pop() && path.pop()) {
            break None;
        }
    }
}

pub fn parent_dir(dir: &Path) -> PathBuf {
    let mut dir = dir.to_owned();
    dir.pop();
    dir
}

pub fn maybe_create_dir<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    match std::fs::create_dir(path) {
        Ok(_) => return Ok(()),
        Err(err) => match err.kind() {
            std::io::ErrorKind::AlreadyExists => return Ok(()),
            _ => return Err(err),
        },
    }
}
