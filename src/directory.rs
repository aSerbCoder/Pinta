pub mod directory {
    use std::env;
    use std::fs::DirEntry;
    use std::path::{Path, PathBuf};

    pub struct Directory {
        pub cur_dir: PathBuf,
        pub dir_contents: Vec<DirEntry>,
    }

    impl Directory {
        fn init_directory() -> PathBuf {
            env::current_dir().expect("Failed to get current directory")
        }

        pub fn directory_contents(path: &Path, show_hidden: bool) -> Vec<DirEntry> {
            path.read_dir()
                .expect("Failed to read directory")
                .filter_map(Result::ok)
                .filter(|entry| {
                    if show_hidden {
                        true
                    } else {
                        entry
                            .file_name()
                            .to_str()
                            .map(|name| !name.starts_with('.'))
                            .unwrap_or(false)
                    }
                })
                .collect()
        }

        pub fn prev_directory(&mut self, show_hidden: bool) {
            env::set_current_dir(self.cur_dir.parent().expect("Failed to get prev directory"))
                .expect("Failed to get set current_dir");
            self.cur_dir = env::current_dir().expect("Failed getting current directory");
            self.dir_contents = Directory::directory_contents(&self.cur_dir, show_hidden);
        }

        pub fn next_directory(&mut self, path: &Path, show_hidden: bool) {
            env::set_current_dir(path).unwrap();
            self.cur_dir = env::current_dir().expect("Failed getting current directory");
            self.dir_contents = Directory::directory_contents(&self.cur_dir, show_hidden);
        }

        pub fn new() -> Self {
            let cur_dir = Directory::init_directory();
            let dir_contents = Directory::directory_contents(&cur_dir, false);
            Directory {
                cur_dir,
                dir_contents,
            }
        }

        pub fn path(&self) -> &Path {
            &self.cur_dir
        }
    }
}
