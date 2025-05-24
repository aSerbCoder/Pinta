pub mod commands {
    use std::{path::Path, process::Command};

    pub fn enter_in_tmux(path: &Path) {
        let session_name = path.file_name().unwrap().to_str().unwrap();
        let path_str = path.to_str().expect("Invalid path");
        let status = Command::new("tmux")
            .args(&[
                "new-session",
                "-d", // start detached
                "-s",
                session_name,
                &format!("cd {} && exec $SHELL", path_str),
            ])
            .status()
            .expect("Failed to start tmux session");

        if status.success() {
            // Attach to the session after creation
            Command::new("tmux")
                .args(&["attach-session", "-t", session_name])
                .status()
                .expect("Failed to attach to tmux session");
        } else {
            eprintln!("Failed to create tmux session");
        }
    }
}
