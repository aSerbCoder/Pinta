use std::{
    env::current_dir,
    fs,
    path::{Path, PathBuf},
};

pub fn get_current_directory_name() -> PathBuf {
    return current_dir()
        .expect("Could not get current directory")
        .to_path_buf();
}

pub fn get_current_directory_contents(dir: &Path, show_hidden: bool) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .unwrap_or_else(|_| panic!("Could not read current directory: {}", dir.display()))
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| {
            if show_hidden {
                true
            } else {
                path.file_name()
                    .and_then(|n| n.to_str())
                    .map(|name| !name.starts_with('.'))
                    .unwrap_or(true)
            }
        })
        .collect()
}
