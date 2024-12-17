use pelite::FileMap;
use std::path::Path;

#[tauri::command]
pub async fn patch_limbus(src_path: String) -> Result<(), String> {
    let limbus_exe = format!("{}/LimbusCompany.exe", src_path);
    patch_limbus_exe(&limbus_exe)?;
    Ok(())
}

fn patch_limbus_exe(exe_path: &str) -> Result<(), String> {
    let path = Path::new(exe_path);
    let map = FileMap::open(path).map_err(|e| format!("Failed to open file: {}", e))?;

    let new_file = steamnvke::drm::strip_drm_from_exe(map.as_ref())
        .map_err(|e| format!("Failed to strip DRM: {}", e))?;

    std::fs::write("./game/LimbusCompany.exe", new_file)
        .map_err(|e| format!("Failed to write file: {}", e))?;
    Ok(())
}
