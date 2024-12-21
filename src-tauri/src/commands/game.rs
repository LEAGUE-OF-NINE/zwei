use std::{env, time::Duration};

use tauri::{AppHandle, Emitter};
use tokio::time::sleep;

pub async fn launch_game(app: AppHandle, launch_args: String, token: String) {
    let mut game_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            log::error!("Failed to get current directory: {}", err);
            app.emit("launch-status", "Failed to get game directory")
                .unwrap();
            return;
        }
    };

    game_dir.push("game"); // Append "game" to the current directory
    let game_path = game_dir.join("LimbusCompany.exe");

    if !game_path.exists() {
        log::error!("Game executable not found at: {}", game_path.display());
        app.emit("launch-status", "Game executable not found")
            .unwrap();
        return;
    }

    app.emit("launch-status", "Launching...").unwrap();

    // Prepare command and arguments
    let cmd = if launch_args.is_empty() {
        vec![game_path.to_string_lossy().to_string()]
    } else {
        match shlex::split(&launch_args.replace("%command%", &game_path.to_string_lossy())) {
            Some(args) => args,
            None => {
                log::error!("Failed to parse launch arguments: {}", launch_args);
                return;
            }
        }
    };

    let command = cmd[0].clone();
    let full_args = cmd[1..].to_vec();

    // Print the command and arguments being executed for debugging
    log::info!("Executing command: {} {}", command, full_args.join(" "));

    // Set the environment variable
    std::env::set_var("LETHE_TOKEN", token.clone());

    #[cfg(target_os = "windows")]
    {
        sandbox::start_game("zweilauncher", &cmd.join(" "));
        app.emit("launch-status", "Launched... Please wait...")
            .unwrap();
        sleep(Duration::from_secs(10)).await;
        app.emit("launch-status", "").unwrap();
    }

    #[cfg(not(target_os = "windows"))]
    {
        app.emit("launch-status", "Only supported on Windows currently")
            .unwrap();
    }
}
