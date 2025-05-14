pub mod directory {
    use std::path::PathBuf;
    use std::process::Command;

    pub struct Directory {
        pub cur_dir: PathBuf,
    }

    impl Directory {
        fn init_directory() -> PathBuf {
            let cur_dir = Command::new("pwd")
                .output()
                .expect("Failed to get current directory");

            let cur_dir_as_string = String::from_utf8(cur_dir.stdout)
                .expect("Invalid UTF-8 output from pwd")
                .trim()
                .to_string();

            PathBuf::from(cur_dir_as_string)
        }

        pub fn new() -> Self {
            let cur_dir = Directory::init_directory();
            Directory { cur_dir }
        }
    }
}
