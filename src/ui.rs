pub mod ui {
    use std::{
        io::{Stdout, stdout},
        path::PathBuf,
    };

    use crossterm::{
        ExecutableCommand,
        style::{Color, Print, SetBackgroundColor},
    };

    pub struct Interface {
        out: Stdout,
    }

    impl Interface {
        pub fn new() -> Interface {
            Interface { out: stdout() }
        }

        pub fn write_cur_directory(mut self, path_buf: PathBuf) {
            let path_str = &path_buf.file_stem().unwrap();

            self.out.execute(SetBackgroundColor(Color::Blue)).unwrap();
            self.out.execute(Print(path_str.to_string_lossy())).unwrap();
        }
    }
}
