use std::io::Read;
use std::process::Command;
use std::{path::Path, process::Stdio};

use chrono::{DateTime, Local, Utc};

#[derive(Default, Debug)]
pub struct TmuxWindow {
    pub index: String,
    pub name: String,
    pub active: bool,
}

#[derive(Default, Debug)]
pub struct TmuxSession {
    pub name: String,
    pub date_created: String,
    pub windows: Vec<TmuxWindow>,
}

pub fn reenter_tmux_session(session_name: &str) {
    let status = Command::new("tmux")
        .args(&["attach-session", "-t", session_name])
        .status()
        .expect("Failed to attach to tmux session");

    if !status.success() {
        eprintln!("Failed to attach to session: {}", session_name);
    }
}

pub fn parse_tmux_timestamp(ts: &str) -> String {
    match ts.parse::<i64>() {
        Ok(epoch) => {
            let dt_utc =
                DateTime::<Utc>::from_timestamp(epoch, 0).expect("Could not create UTC DateTime");

            let dt_local: DateTime<Local> = dt_utc.with_timezone(&Local);

            dt_local.format("%Y-%m-%d %H:%M:%S").to_string()
        }
        Err(_) => "Invalid timestamp".to_string(),
    }
}

pub fn enter_in_tmux(path: &Path) {
    let session_name = path
        .file_name()
        .and_then(|f| f.to_str())
        .expect("Invalid path for session name");

    let path_str = path.to_str().expect("Invalid path");

    let status = Command::new("tmux")
        .args(&[
            "new-session",
            "-d",
            "-s",
            session_name,
            &format!("cd {} && exec $SHELL", path_str),
        ])
        .status()
        .expect("Failed to start tmux session");

    if status.success() {
        Command::new("tmux")
            .args(&["attach-session", "-t", session_name])
            .status()
            .expect("Failed to attach to tmux session");
    } else {
        eprintln!("Failed to create tmux session");
    }
}

pub fn list_tmux_sessions() -> Vec<TmuxSession> {
    let mut sessions_child = Command::new("tmux")
        .args(&["list-sessions", "-F", "#{session_name}:#{session_created}"])
        .stderr(Stdio::null())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run tmux list-sessions");

    let mut sessions_stdout = sessions_child
        .stdout
        .take()
        .expect("Failed to get tmux list-sessions stdout");

    let mut buf = String::new();
    sessions_stdout
        .read_to_string(&mut buf)
        .expect("Failed to read stdout");

    let mut sessions = Vec::new();

    for line in buf.lines() {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() != 2 {
            continue;
        }

        let session_name = parts[0].to_string();
        let date_created = parse_tmux_timestamp(parts[1]);

        let mut windows_child = Command::new("tmux")
            .args(&[
                "list-windows",
                "-t",
                &session_name,
                "-F",
                "#{window_index}:#{window_name}:#{window_active}",
            ])
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to run tmux list-windows");

        let mut windows_stdout = windows_child
            .stdout
            .take()
            .expect("Failed to get tmux list-windows stdout");

        let mut windows_buf = String::new();
        windows_stdout
            .read_to_string(&mut windows_buf)
            .expect("Failed to read windows stdout");

        let mut windows = Vec::new();
        for win_line in windows_buf.lines() {
            let win_parts: Vec<&str> = win_line.splitn(3, ':').collect();
            if win_parts.len() != 3 {
                continue;
            }

            windows.push(TmuxWindow {
                index: win_parts[0].to_string(),
                name: win_parts[1].to_string(),
                active: win_parts[2] == "1",
            });
        }

        sessions.push(TmuxSession {
            name: session_name,
            date_created,
            windows,
        });
    }

    sessions
}
