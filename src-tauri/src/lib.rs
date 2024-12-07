use futures::stream::StreamExt;
use pelite::FileMap;
use reqwest::Client;
use std::fs;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
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

#[tauri::command]
fn add_distribute_files() -> Result<(), String> {
    let source_dir = Path::new("./distribute");
    let target_dir = Path::new("./game");

    if !target_dir.exists() {
        std::fs::create_dir(target_dir)
            .map_err(|e| format!("Failed to create target directory: {}", e))?;
    }

    copy_directory_recursively(source_dir, target_dir)
        .map_err(|err| format!("Failed to copy directory recursively: {}", err))?;
    println!(
        "Files copied from {} to {}",
        source_dir.display(),
        target_dir.display()
    );
    Ok(())
}

fn copy_directory_recursively(source: &Path, destination: &Path) -> std::io::Result<()> {
    if !destination.exists() {
        std::fs::create_dir(destination)?;
    }
    for entry in std::fs::read_dir(source)? {
        let entry = entry?;
        let dest_path = destination.join(entry.file_name());
        if entry.path().is_dir() {
            copy_directory_recursively(&entry.path(), &dest_path)?;
        } else {
            std::fs::copy(entry.path(), &dest_path)?;
        }
    }
    Ok(())
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
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            clone_folder_to_game,
            download_and_extract_bepinex,
            download_and_install_lethe,
            patch_limbus,
            add_distribute_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
