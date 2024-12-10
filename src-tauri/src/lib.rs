use futures::stream::StreamExt;
use pelite::FileMap;
use reqwest::Client;
use serde_json::Value;
use std::path::Path;
use std::process::Command;
use std::{env, fs};
use tauri_plugin_deep_link::DeepLinkExt;
use tauri_plugin_store::StoreExt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::task;
use zip::read::ZipArchive;

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
    let directory = "game/bepinex/plugins";
    let destination = format!("{}/Lethe.dll", directory);

    std::fs::create_dir_all(directory)
        .map_err(|err| format!("Failed to create dirs recursively: {}", err))?;

    download_file(url, &destination)
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

async fn launch_game(launch_args: String, use_sandbox: bool, sandbox_path: String, token: String) {
    let game_dir = "./game";
    let game_path = format!("{}/LimbusCompany.exe", game_dir);

    let mut command = if use_sandbox {
        let mut sandbox_command = Command::new(sandbox_path);
        sandbox_command.arg("./LimbusCompany.exe");
        sandbox_command
    } else {
        Command::new(&game_path)
    };

    // Set the working directory
    command.current_dir(game_dir);

    command.env("LETHE_TOKEN", &token);

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

    let mut builder = tauri::Builder::default();
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|_app, argv, _cwd| {
          println!("a new app instance was opened with {argv:?} and the deep link event was already triggered");
        }));
    }

    builder
        .plugin(tauri_plugin_deep_link::init())
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
        ])
        .setup(|app| {
            // Create a new store or load the existing one
            // this also put the store in the app's resource table
            // so your following calls `store` calls (from both rust and js)
            // will reuse the same store
            let store = app.store("store.json")?;

            #[cfg(any(windows, target_os = "linux"))]
            {
                app.deep_link().register_all()?;
            }

            app.deep_link().on_open_url(move |event| {
                let launch_args: String = store.get("launchArgs").unwrap_or("".into()).to_string();
                let use_sandbox: bool = store
                    .get("sandbox")
                    .unwrap_or(Value::Bool(false))
                    .as_bool()
                    .unwrap_or(false);
                let sandbox_path: String =
                    store.get("sandboxPath").unwrap_or("".into()).to_string();

                let urls = event.urls();
                let owned_urls: Vec<_> = urls.into_iter().collect(); // Due to rust ownership system we must fully own every url here

                if let Some(first_url) = owned_urls.first() {
                    if let Some(token) = first_url.query() {
                        let launch_args_clone = launch_args.clone();
                        let sandbox_path_clone = sandbox_path.clone();
                        let token_clone = token.to_string(); // Another owned string conversion here

                        // Delegate launch game to tokio to prevent blocking the main thread
                        task::spawn(async move {
                            launch_game(
                                launch_args_clone,
                                use_sandbox,
                                sandbox_path_clone,
                                token_clone,
                            )
                            .await;
                        });
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
