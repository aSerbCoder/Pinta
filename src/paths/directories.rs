use std::{
    env::current_dir,
    fs, io,
    path::{Path, PathBuf},
};

pub fn get_current_directory_name() -> PathBuf {
    return current_dir()
        .expect("Could not get current directory")
        .to_path_buf();
}

pub fn get_current_directory_contents(dir: &Path) -> Vec<PathBuf> {
    return fs::read_dir(dir)
        .expect("Could not get read current directory")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .expect("Could not get contents inside of current directory");
}
