use reqwest::blocking::get;
use std::fs::File;
use std::fs::{self, create_dir_all};
use std::io::{self};
use std::path::Path;
use zip::read::ZipArchive;

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

#[tauri::command]
fn download_and_extract_bepinex() {
    let url = "https://builds.bepinex.dev/projects/bepinex_be/577/BepInEx_UnityIL2CPP_x64_ec79ad0_6.0.0-be.577.zip";
    let zip_path = "BepInEx_UnityIL2CPP_x64_ec79ad0_6.0.0-be.577.zip";
    let extract_to = "game";

    if let Err(e) = download_file(url, zip_path) {
        eprintln!("Failed to download the file: {}", e);
        return;
    }
    println!("Downloaded ZIP file successfully.");

    if let Err(e) = unzip_file(zip_path, extract_to) {
        eprintln!("Failed to unzip the file: {}", e);
        return;
    }
    println!("Unzipped the file successfully.");
}

fn download_file(url: &str, destination: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(url)?;
    let mut file = File::create(destination)?;
    io::copy(&mut response.bytes()?.as_ref(), &mut file)?;
    Ok(())
}

fn unzip_file(zip_path: &str, extract_to: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    create_dir_all(extract_to)?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = Path::new(extract_to).join(file.name());

        if file.name().ends_with('/') {
            create_dir_all(outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                create_dir_all(parent)?;
            }
            let mut outfile = File::create(outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            clone_folder_to_game,
            download_and_extract_bepinex
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
