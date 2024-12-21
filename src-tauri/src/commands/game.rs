use std::{env, path::PathBuf};

use tauri::{AppHandle, Emitter};
use tauri_plugin_shell::{process::CommandEvent, ShellExt};

pub async fn launch_game(
    app: AppHandle,
    launch_args: String,
    token: String,
    is_sandbox: bool,
    sandbox_path: String,
) {
    log::info!("Starting game launch process. is_sandbox: {}", is_sandbox);

    let mut game_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            log::error!("Failed to get current directory: {}", err);
            app.emit("launch-status", "Failed to get game directory")
                .unwrap();
            return;
        }
    };

    log::info!("Current working directory: {}", game_dir.display());
    game_dir.push("game");
    log::info!("Game working directory: {}", game_dir.display());
    let game_path = game_dir.join("LimbusCompany.exe");

    if !game_path.exists() {
        log::error!("Game executable not found at: {}", game_path.display());
        app.emit("launch-status", "Game executable not found")
            .unwrap();
        return;
    }

    app.emit("launch-status", "Launching...").unwrap();
    log::info!("Resolved game executable path: {}", game_path.display());

    let cmd = if launch_args.is_empty() {
        vec![game_path.clone()]
    } else {
        match shlex::split(&launch_args.replace("%command%", &game_path.to_string_lossy())) {
            Some(args) => args.into_iter().map(PathBuf::from).collect(),
            None => {
                log::error!("Failed to parse launch arguments: {}", launch_args);
                return;
            }
        }
    };

    let command = cmd[0].clone();
    let full_args = cmd[1..].to_vec();

    // Adjust command and arguments if sandbox is enabled
    let (command, full_args) = if is_sandbox {
        (
            PathBuf::from(sandbox_path),
            vec![PathBuf::from("LimbusCompany.exe")]
                .into_iter()
                .chain(full_args)
                .collect(),
        )
    } else {
        (command, full_args)
    };

    // Print the command and arguments being executed for debugging
    log::info!(
        "Executing command: {} {}",
        command.display(),
        full_args
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );

    let shell = app.shell();
    match shell
        .command(&command)
        .current_dir(game_dir)
        .env("LETHE_TOKEN", token.clone())
        .args(
            full_args
                .iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect::<Vec<String>>(),
        )
        .spawn()
    {
        Ok((mut rx, _child)) => {
            // Emit event immediately upon spawning the process
            app.emit("launch-status", "Game launched successfully!")
                .unwrap();
            log::info!("Game process started successfully.");
            // Listen for command events (stdout, stderr, etc.)
            let app_handle = app.clone();
            tauri::async_runtime::spawn(async move {
                while let Some(event) = rx.recv().await {
                    match event {
                        CommandEvent::Stdout(line) => {
                            let output = String::from_utf8_lossy(&line);
                            log::info!("Game output: {}", output);
                            app_handle.emit("game-stdout", output.to_string()).unwrap();
                        }
                        CommandEvent::Stderr(line) => {
                            let error = String::from_utf8_lossy(&line);
                            log::error!("Game error: {}", error);
                            app_handle.emit("game-stderr", error.to_string()).unwrap();
                        }
                        _ => {
                            app_handle.emit("launch-status", "").unwrap();
                        }
                    }
                }
            });
        }
        Err(err) => {
            app.emit(
                "launch-status",
                format!("Failed to launch the game: {}", err),
            )
            .unwrap();
            log::error!("Failed to launch the game: {}", err);
        }
    }
}
