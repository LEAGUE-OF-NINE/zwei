use crate::commands::checksum;
use std::path::PathBuf;
use std::{env, fs, path::Path};

fn get_cmd_path() -> Option<String> {
    if let Ok(system_root) = env::var("SystemRoot") {
        let cmd_path = format!("{}\\System32\\cmd.exe", system_root);
        if std::path::Path::new(&cmd_path).exists() {
            return Some(cmd_path);
        }
    }
    None
}

// Sets up an appcontainer with a placeholder until limbus is installed
fn setup_app_container() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let cmd_path = get_cmd_path().ok_or("cmd.exe not found")?;
        sandbox::appcontainer::Profile::new("zweilauncher", &cmd_path)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("AppContainer only works on Windows".to_string())
    }
}

#[tauri::command]
pub async fn clone_folder_to_game(src_path: String) -> Result<(), String> {
    setup_app_container()?; // This is needed because otherwise the game deletes itself when launching for the first time

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

    let src_path = PathBuf::from(src);
    let dst_path = PathBuf::from(dest);
    let ok = checksum::get_manifest()
        .await
        .map_err(|e| e.to_string())?
        .copy_to_folder(&src_path, &dst_path);
    if ok.is_err() {
        // remove LimbusCompany.exe if integrity failed to force the game to be properly copied before launch
        if let Err(err) = fs::remove_file(dst_path) {
            log::error!("Failed to delete LimbusCompany.exe on error: {}", err);
        }
    }

    ok.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn open_game_folder() -> Result<(), String> {
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

fn get_lethe_limbus_folder_location() -> String {
    let local_appdata = env::var("LOCALAPPDATA").expect("Failed to get LOCALAPPDATA");
    local_appdata + "/Packages/zweilauncher/AC/game"
}

#[tauri::command]
pub async fn check_lethe_limbus_up_to_date() -> Result<bool, String> {
    let lethe_limbus = get_lethe_limbus_folder_location();
    let lethe_path = PathBuf::from(lethe_limbus);
    checksum::get_manifest()
        .await
        .map_err(|e| e.to_string())?
        .check_is_up_to_date(&lethe_path)
        .map_err(|e| e.to_string())
}
