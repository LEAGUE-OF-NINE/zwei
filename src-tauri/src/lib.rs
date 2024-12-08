use futures::stream::StreamExt;
use pelite::FileMap;
use reqwest::Client;
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::{env, fs};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use utils::open_browser;
use warp::Filter;
use zip::read::ZipArchive;
mod utils;

#[tauri::command]
async fn start_login_server(port: u16, launch_args: String, use_sandbox: bool) -> String {
    println!("Recieved args {}", launch_args);
    // Create a channel to trigger server shutdown
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();

    // Wrap the Sender in an Arc and Mutex for shared access
    let shutdown_tx = Arc::new(Mutex::new(Some(shutdown_tx)));

    tokio::spawn(async move {
        let cors = warp::cors()
            .allow_any_origin()
            .allow_methods(vec!["GET", "POST"])
            .allow_headers(vec!["Content-Type", "Authorization"]);

        // Define a POST route that expects a JSON body with the token
        let shutdown_route = warp::post()
            .and(warp::path("auth"))
            .and(warp::path("login"))
            .and(warp::body::json())
            .map({
                let shutdown_tx = shutdown_tx.clone();
                move |body: serde_json::Value| {
                    if let Some(received_token) = body.get("token").and_then(|t| t.as_str()) {
                        println!("Received token: {}", received_token);

                        let token = received_token.to_string();
                        let launch_args = launch_args.clone();

                        // Send the shutdown signal
                        let mut shutdown_tx = shutdown_tx.lock().unwrap();
                        if let Some(tx) = shutdown_tx.take() {
                            let _ = tx.send(());
                        }

                        tokio::task::spawn_blocking(move || {
                            launch_game(&token, &launch_args, use_sandbox);
                        });

                        return warp::reply::html("Server shutting down...");
                    }
                    warp::reply::html("Invalid token or no token provided.")
                }
            });

        open_browser(&format!(
            "https://api.lethelc.site/auth/login?port={}",
            port
        ));
        // Apply CORS and run the server on localhost:3030
        warp::serve(shutdown_route.with(cors))
            .run(([127, 0, 0, 1], port))
            .await;

        // Wait for shutdown signal and exit
        shutdown_rx.await.unwrap();
    });
    "Server is running! Send a Login request to shut it down.".to_string()
}

#[tauri::command]
async fn patch_limbus(src_path: String) -> Result<(), String> {
    let limbus_exe = src_path + "/LimbusCompany.exe";
    patch_limbus_exe(limbus_exe)?;
    Ok(())
}

#[tauri::command]
async fn download_and_extract_bepinex() -> Result<(), String> {
    let url = "https://builds.bepinex.dev/projects/bepinex_be/577/BepInEx_UnityIL2CPP_x64_ec79ad0_6.0.0-be.577.zip";
    let zip_path = "BepInEx_UnityIL2CPP_x64_ec79ad0_6.0.0-be.577.zip";
    let extract_to = "game";

    download_file(url, zip_path)
        .await
        .map_err(|e| format!("Error downloading file: {}", e))?;
    unzip_file(zip_path, extract_to).map_err(|e| format!("Error unzipping file: {}", e))?;
    Ok(())
}

#[tauri::command]
async fn download_and_install_lethe() -> Result<(), String> {
    let url = "https://api.lethelc.site/Lethe.dll";
    let destination = "game/bepinex/plugins/Lethe.dll";

    download_file(url, destination)
        .await
        .map_err(|e| format!("Failed to download the file: {}", e))?;

    Ok(())
}

async fn download_file(url: &str, destination: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(format!("Failed to download file: {}", response.status()).into());
    }

    let mut file = File::create(destination).await?;
    let mut content = response.bytes_stream();

    while let Some(chunk) = content.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
    }

    Ok(())
}

fn unzip_file(zip_path: &str, extract_to: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    std::fs::create_dir_all(extract_to)?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = Path::new(extract_to).join(file.name());

        if file.name().ends_with('/') {
            std::fs::create_dir_all(outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut outfile = std::fs::File::create(outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

#[tauri::command]
async fn clone_folder_to_game(src_path: String) -> Result<(), String> {
    let src = Path::new(&src_path);
    let dest = Path::new("./game");

    copy_dir_all(src, dest).map_err(|e| format!("Failed to clone folder: {}", e))?;
    Ok(())
}

#[tauri::command]
fn open_game_folder() -> Result<(), String> {
    // Get the current working directory
    let current_dir =
        std::env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;

    // Construct the full path to ./game from the current directory
    let game_dir = current_dir.join("game");

    // Check if the directory exists
    if !game_dir.is_dir() {
        return Err(format!(
            "The directory '{}' does not exist.",
            game_dir.display()
        ));
    }

    // Use platform-specific commands to open the directory
    #[cfg(target_os = "windows")]
    let mut command = std::process::Command::new("explorer");
    #[cfg(target_os = "windows")]
    command.arg(game_dir);

    #[cfg(target_os = "macos")]
    let mut command = std::process::Command::new("open");
    #[cfg(target_os = "macos")]
    command.arg(game_dir);

    #[cfg(target_os = "linux")]
    let mut command = std::process::Command::new("xdg-open");
    #[cfg(target_os = "linux")]
    command.arg(game_dir);

    command
        .spawn()
        .map_err(|e| format!("Failed to open game directory: {}", e))?;

    Ok(())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn launch_game(token: &str, launch_args: &str, use_sandbox: bool) {
    let game_path = "./game/LimbusCompany.exe";

    let mut command = if use_sandbox {
        let sandboxie_path = "C:\\Program Files\\Sandboxie\\Start.exe";
        let mut sandbox_command = Command::new(sandboxie_path);
        sandbox_command.arg(game_path); // Add the game path as an argument for Sandboxie
        sandbox_command
    } else {
        Command::new(game_path) // Directly launch the game if not using Sandboxie
    };

    // Set environment variables
    command.env("LETHE_TOKEN", token);

    // Add launch arguments
    let args: Vec<&str> = launch_args.split_whitespace().collect();
    command.args(&args);

    match command.spawn() {
        Ok(mut child) => {
            println!("Game launched with PID: {}", child.id());
            if let Err(err) = child.wait() {
                eprintln!("Failed to wait on process: {}", err);
            }
        }
        Err(err) => {
            eprintln!("Failed to launch the game: {}", err);
        }
    }
}

fn patch_limbus_exe(exe_path: String) -> Result<(), String> {
    let path = Path::new(&exe_path);
    let map = FileMap::open(path).map_err(|e| format!("Failed to open file: {}", e))?;

    let new_file = steamnvke::drm::strip_drm_from_exe(map.as_ref())
        .map_err(|e| format!("Failed to strip DRM: {}", e))?;

    fs::write("./game/LimbusCompany.exe", new_file)
        .map_err(|e| format!("Failed to write LimbusCompany file: {}", e))?;

    println!("Successfully patched and saved LimbusCompany.exe.");
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .expect("Failed to determine executable directory");
    std::env::set_current_dir(&exe_dir)
        .expect("Failed to set current directory to executable's directory");

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            clone_folder_to_game,
            download_and_extract_bepinex,
            download_and_install_lethe,
            patch_limbus,
            open_game_folder,
            start_login_server
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
