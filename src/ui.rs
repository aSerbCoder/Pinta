pub mod ui {
    use std::{
        io::{Stdout, stdout},
        path::Path,
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

        pub fn write_cur_directory(&mut self, path: &Path) {
            if let Some(stem) = path.file_stem() {
                self.out.execute(SetBackgroundColor(Color::Blue)).unwrap();
                self.out.execute(Print(stem.to_string_lossy())).unwrap();
            }
        }
    }
}
