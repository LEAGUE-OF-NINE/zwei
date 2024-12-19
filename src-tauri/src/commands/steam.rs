#[tauri::command]
pub async fn steam_limbus_location() -> String {
    #[cfg(target_os = "windows")]
    {
        use windows_registry::CURRENT_USER;
        CURRENT_USER
            .create("Software\\Valve\\Steam")
            .and_then(|key| key.get_string("SteamPath"))
            .map(|path| {
                std::path::Path::new(&path)
                    .join("steamapps/common/Limbus Company/")
                    .to_string_lossy()
                    .to_string()
            })
            .unwrap_or_default()
    }

    #[cfg(not(target_os = "windows"))]
    {
        String::new()
    }
}