use std::path::Path;
use windows_registry::CURRENT_USER;

#[tauri::command]
pub async fn steam_limbus_location() -> String {
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
