pub mod ui {
    use std::{
        fs::DirEntry,
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

        fn println_entry(&mut self, e: &DirEntry) {
            let stem = e.file_name();
            self.out
                .execute(SetBackgroundColor(Color::DarkGrey))
                .unwrap();
            self.out.execute(Print(stem.to_string_lossy())).unwrap();
            self.out
                .execute(SetBackgroundColor(Color::Rgb { r: 0, g: 0, b: 0 }))
                .unwrap();
            self.out.execute(Print("\r\n")).unwrap();
        }

        fn print_entry(&mut self, e: &DirEntry) {
            let stem = e.file_name();
            self.out
                .execute(SetBackgroundColor(Color::DarkGrey))
                .unwrap();
            self.out.execute(Print(stem.to_string_lossy())).unwrap();
            self.out
                .execute(SetBackgroundColor(Color::Rgb { r: 0, g: 0, b: 0 }))
                .unwrap();
        }

        pub fn write_cur_directory(&mut self, path: &Path) {
            if let Some(stem) = path.file_stem() {
                self.out.execute(SetBackgroundColor(Color::Blue)).unwrap();
                self.out.execute(Print(stem.to_string_lossy())).unwrap();
                self.out
                    .execute(SetBackgroundColor(Color::Rgb { r: 0, g: 0, b: 0 }))
                    .unwrap();
                self.out.execute(Print("\r\n")).unwrap();
            }
        }

        pub fn write_directory_contents(&mut self, contents: &Vec<DirEntry>) {
            for (i, entry) in contents.iter().enumerate() {
                if i != contents.len() - 1 {
                    self.println_entry(entry);
                } else {
                    self.print_entry(entry);
                }
            }
        }
    }
}
