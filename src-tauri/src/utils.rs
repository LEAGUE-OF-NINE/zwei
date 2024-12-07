use std::{env, process::Command};

pub fn open_browser(url: &str) {
    if cfg!(target_os = "windows") {
        // Windows
        Command::new("cmd")
            .arg("/C")
            .arg(format!("start {}", url))
            .spawn()
            .expect("Failed to open browser");
    } else if cfg!(target_os = "macos") {
        // macOS
        Command::new("open")
            .arg(url)
            .spawn()
            .expect("Failed to open browser");
    } else if cfg!(target_os = "linux") {
        // Linux
        let browser_command = if let Ok(desktop_env) = env::var("XDG_SESSION_DESKTOP") {
            if desktop_env.to_lowercase() == "gnome" {
                "gnome-open"
            } else {
                "xdg-open"
            }
        } else {
            "xdg-open"
        };

        Command::new(browser_command)
            .arg(url)
            .spawn()
            .expect("Failed to open browser");
    } else {
        eprintln!("Unsupported OS");
    }
}
