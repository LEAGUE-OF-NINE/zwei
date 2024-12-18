use std::time::Duration;

use tauri::{AppHandle, Emitter};
use tokio::time::sleep;

pub async fn launch_game(app: AppHandle, launch_args: String, token: String) {
    let game_dir = "./game";
    let game_path = format!("{}/LimbusCompany.exe", game_dir);
    app.emit("launch-status", "Launching...").unwrap();

    // Prepare command and arguments
    let cmd = if launch_args.is_empty() {
        vec![game_path.clone()]
    } else {
        match shlex::split(&launch_args.replace("%command%", &game_path)) {
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

    std::env::set_var("LETHE_TOKEN", token.clone());
    sandbox::start_game("zweilauncher", &cmd[0..].join(" "));
    app.emit("launch-status", "Launched... Please wait...")
        .unwrap();
    sleep(Duration::from_secs(10)).await;
    app.emit("launch-status", "").unwrap();
}
