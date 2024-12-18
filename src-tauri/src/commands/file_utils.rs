use std::{env, fs, path::Path, time::SystemTime};

use super::steam::steam_limbus_location;

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
    let cmd_path = get_cmd_path().ok_or("cmd.exe not found")?;
    sandbox::appcontainer::Profile::new("zweilauncher", &cmd_path).map_err(|e| e.to_string())?;
    Ok(())
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

    copy_dir_all(src, dest).map_err(|e| format!("Failed to clone folder: {}", e))?;

    Ok(())
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

fn get_file_modified_time(path: &str) -> std::io::Result<SystemTime> {
    let metadata = fs::metadata(path)?;
    metadata.modified()
}

fn get_lethe_folder_location() -> String {
    let local_appdata = env::var("LOCALAPPDATA").expect("Failed to get LOCALAPPDATA");
    local_appdata + "/Packages/zweilauncher/AC"
}

#[tauri::command]
pub async fn check_new_limbus_version() -> Result<bool, String> {
    let mut steam_limbus = steam_limbus_location().await;
    steam_limbus += "/LimbusCompany.exe";
    let steam_limbus_modified_date =
        get_file_modified_time(&steam_limbus).map_err(|e| e.to_string())?;

    let lethe_limbus = get_lethe_folder_location() + "/game/LimbusCompany.exe";
    println!("{}", lethe_limbus);
    let lethe_limbus_modified_date =
        get_file_modified_time(&lethe_limbus).map_err(|e| e.to_string())?;

    Ok(lethe_limbus_modified_date < steam_limbus_modified_date)
}
