use std::{fs, path::Path};

#[tauri::command]
fn clone_folder_to_game(src_path: String) -> Result<(), String> {
    let src = Path::new(&src_path);
    let dest = Path::new("./game");

    if !dest.exists() {
        fs::create_dir_all(dest)
            .map_err(|e| format!("Failed to create destination directory: {}", e))?;
    }

    if src.is_dir() {
        for entry in
            fs::read_dir(src).map_err(|e| format!("Failed to read source directory: {}", e))?
        {
            let entry = entry.map_err(|e| format!("Error reading entry: {}", e))?;
            let file_name = entry.file_name();
            let destination = dest.join(file_name);

            if entry.path().is_file() {
                fs::copy(entry.path(), destination)
                    .map_err(|e| format!("Failed to copy file: {}", e))?;
            } else if entry.path().is_dir() {
                fs::create_dir_all(&destination)
                    .map_err(|e| format!("Failed to create subdirectory: {}", e))?;
                clone_folder_to_game(entry.path().to_str().unwrap().to_string())?;
            }
        }
        Ok(())
    } else {
        Err("Source path is not a valid directory".into())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![clone_folder_to_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
