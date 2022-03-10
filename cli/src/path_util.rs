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
