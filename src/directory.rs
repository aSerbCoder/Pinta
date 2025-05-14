pub mod directory {
    use std::env;
    use std::path::{Path, PathBuf};

    pub struct Directory {
        pub cur_dir: PathBuf,
    }

    impl Directory {
        fn init_directory() -> PathBuf {
            env::current_dir().expect("Failed to get current directory")
        }

        pub fn new() -> Self {
            let cur_dir = Directory::init_directory();
            Directory { cur_dir }
        }

        pub fn path(&self) -> &Path {
            &self.cur_dir
        }
    }
}
