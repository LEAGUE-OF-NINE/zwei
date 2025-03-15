use futures::stream::StreamExt;
use reqwest::Client;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use zip::read::ZipArchive;

#[tauri::command]
pub async fn download_and_extract_bepinex() -> Result<(), String> {
    let url = "https://lethelc.site/libraries/BepInEx577.zip";
    let zip_path = "BepInEx_UnityIL2CPP_x64_ec79ad0_6.0.0-be.577.zip";
    let extract_to = "./game";

    download_file(url, zip_path)
        .await
        .map_err(|e| format!("Error downloading file: {}", e))?;
    unzip_file(zip_path, extract_to).map_err(|e| format!("Error unzipping file: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn download_and_install_lethe() -> Result<(), String> {
    let url = "https://api.lethelc.site/Lethe.dll";
    let directory = "./game/bepinex/plugins";
    let destination = format!("{}/Lethe.dll", directory);

    std::fs::create_dir_all(directory)
        .map_err(|err| format!("Failed to create dirs recursively: {}", err))?;

    download_file(url, &destination)
        .await
        .map_err(|e| format!("Failed to download the file: {}", e))?;

    let url = "https://api.lethelc.site/libraries/BepInEx.cfg";
    let directory = "./game/bepinex/config";
    let destination = format!("{}/BepInEx.cfg", directory);

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
    let mut file = File::create(destination).await?;

    let mut content = response.bytes_stream();
    while let Some(chunk) = content.next().await {
        file.write_all(&chunk?).await?;
    }

    Ok(())
}

fn unzip_file(zip_path: &str, extract_to: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Unzipping to: {}", extract_to);
    let file = std::fs::File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;
    std::fs::create_dir_all(extract_to)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_path = Path::new(file.name());
        let outpath = Path::new(extract_to).join(file_path);

        // Ensure the path is valid and doesn't escape the intended directory.
        if !outpath.starts_with(extract_to) {
            return Err(format!("Unsafe path detected: {:?}", outpath).into());
        }

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut outfile = std::fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}
