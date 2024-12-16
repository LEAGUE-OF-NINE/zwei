use futures::stream::StreamExt;
use pelite::FileMap;
use reqwest::Client;
use std::path::{Path, PathBuf};
use std::{env, fs};
use tauri::{AppHandle, Emitter};
use tauri_plugin_deep_link::DeepLinkExt;
use tauri_plugin_store::StoreExt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::task;
use utils::extract_value;
use windows_registry::CURRENT_USER;
use zip::read::ZipArchive;
mod utils;

#[tauri::command]
async fn steam_limbus_location() -> String {
    CURRENT_USER
        .create("Software\\Valve\\Steam")
        .and_then(|key| key.get_string("SteamPath"))
        .map(|path| {
            Path::new(&path)
                .join("steamapps/common/Limbus Company/")
                .to_string_lossy()
                .to_string()
        })
        .unwrap_or_default()
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

    let limbus_path = src.join("LimbusCompany.exe");
    let limbus_data_path = src.join("LimbusCompany_data");

    if !limbus_path.exists() || !limbus_path.is_file() {
        return Err("LimbusCompany.exe not found in the source directory.".to_string());
    }

    if !limbus_data_path.exists() || !limbus_data_path.is_dir() {
        return Err("LimbusCompany_Data not found in the source directory.".to_string());
    }

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

async fn launch_game(
    app: AppHandle,
    launch_args: String,
    use_sandbox: bool,
    sandbox_path: String,
    token: String,
) {
    let game_dir = "./game";
    let game_path = format!("{}/LimbusCompany.exe", game_dir);
    log::info!("RECEIVED SANDBOX PATH: {}", sandbox_path);
    log::info!("RECEIVED SANDBOX BOOL: {}", use_sandbox);
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
    app.emit("launch-status", "").unwrap();
}

fn patch_limbus_exe(exe_path: String) -> Result<(), String> {
    let path = Path::new(&exe_path);
    let map = FileMap::open(path).map_err(|e| format!("Failed to open file: {}", e))?;

    let new_file = steamnvke::drm::strip_drm_from_exe(map.as_ref())
        .map_err(|e| format!("Failed to strip DRM: {}", e))?;

    fs::write("./game/LimbusCompany.exe", new_file)
        .map_err(|e| format!("Failed to write LimbusCompany file: {}", e))?;

    log::info!("Successfully patched and saved LimbusCompany.exe.");
    Ok(())
}

fn set_current_dir_to_appdata() {
    let local_appdata = env::var("LOCALAPPDATA").expect("Failed to get LOCALAPPDATA");
    let target_dir: PathBuf = PathBuf::from(&local_appdata).join("Packages/zweilauncher/AC");
    fs::create_dir_all(&target_dir).expect("Failed to create target directory");
    env::set_current_dir(&target_dir).expect("Failed to set current directory");
    log::info!("Current directory set to: {}", target_dir.display());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    set_current_dir_to_appdata();

    let mut builder = tauri::Builder::default().plugin(
        tauri_plugin_log::Builder::new()
            .target(tauri_plugin_log::Target::new(
                tauri_plugin_log::TargetKind::LogDir {
                    file_name: Some("logs".to_string()),
                },
            ))
            .build(),
    );
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|_app, argv, _cwd| {
          log::info!("a new app instance was opened with {argv:?} and the deep link event was already triggered");
        }));
    }

    builder
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_deep_link::init())
        .invoke_handler(tauri::generate_handler![
            steam_limbus_location,
            download_and_extract_bepinex,
            download_and_install_lethe,
            patch_limbus,
            open_game_folder,
            clone_folder_to_game,
        ])
        .setup(|app| {
            // Create a new store or load the existing one
            // this also put the store in the app's resource table
            // so your following calls `store` calls (from both rust and js)
            // will reuse the same store
            let store = app.store("store.json")?;

            let app_handle = app.handle().clone();
            #[cfg(any(windows, target_os = "linux"))]
            {
                app.deep_link().register_all()?;
            }

            app.deep_link().on_open_url(move |event| {
                let handle_clone = app_handle.clone();
                let launch_args: String = extract_value(&store, "launchArgs", "".to_string());
                let use_sandbox: bool = extract_value(&store, "sandbox", false);
                let sandbox_path: String = extract_value(&store, "sandboxPath", "".to_string());

                let urls = event.urls();
                let owned_urls: Vec<_> = urls.into_iter().collect(); // Due to rust ownership system we must fully own every url here

                if let Some(first_url) = owned_urls.first() {
                    if let Some(token) = first_url.query() {
                        let launch_args_clone = launch_args.clone();
                        let sandbox_path_clone = sandbox_path.clone();
                        let token_clone = token.to_string(); // Another owned string conversion here
                        let handle_clone = handle_clone.clone(); // Clone the handle again for the async block

                        // Delegate launch game to tokio to prevent blocking the main thread
                        task::spawn(async move {
                            launch_game(
                                handle_clone,
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
