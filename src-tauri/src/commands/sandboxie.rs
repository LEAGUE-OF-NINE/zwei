use super::file_utils::{get_cache_directories, get_lethe_plugins_folder_location};
use crate::{
    commands::file_utils::get_lethe_limbus_folder_location,
    utf16le_utils::{add_directive, remove_directive},
    utils::detect_sandboxie_ini,
};

// Block User Registry Keys
#[tauri::command]
pub async fn sandboxie_block_user_registry() -> Result<String, String> {
    // Locate the Sandboxie.ini file
    let sandbox_ini = detect_sandboxie_ini().ok_or_else(|| {
        log::error!("Sandboxie.ini not found");
        "Sandboxie.ini not found".to_string()
    })?;
    let sandbox_config_path = sandbox_ini.to_string_lossy().to_string();

    // Define the registry path to block (example registry key path)
    let registry_path = "HKEY_CURRENT_USER\\Software";

    // Construct the directive
    let directive = format!("WriteKeyPath={}", registry_path);

    // Add the directive to block the registry path
    add_directive(&sandbox_config_path, &directive).map_err(|e| {
        log::error!("Failed to update Sandboxie.ini: {}", e);
        format!("Failed to update Sandboxie.ini: {}", e)
    })?;

    log::info!("Successfully added registry block directive to Sandboxie.ini");
    Ok(format!(
        "Successfully updated Sandboxie.ini at: {}",
        sandbox_config_path
    ))
}

// Unblock User Registry Keys
#[tauri::command]
pub async fn sandboxie_unblock_user_registry() -> Result<String, String> {
    // Locate the Sandboxie.ini file
    let sandbox_ini = detect_sandboxie_ini().ok_or_else(|| {
        log::error!("Sandboxie.ini not found");
        "Sandboxie.ini not found".to_string()
    })?;
    let sandbox_config_path = sandbox_ini.to_string_lossy().to_string();

    // Define the registry path to unblock (example registry key path)
    let registry_path = "HKEY_CURRENT_USER\\Software";

    // Construct the directive
    let directive = format!("WriteKeyPath={}", registry_path);

    // Remove the directive to unblock the registry path
    remove_directive(&sandbox_config_path, &directive).map_err(|e| {
        log::error!("Failed to update Sandboxie.ini: {}", e);
        format!("Failed to update Sandboxie.ini: {}", e)
    })?;

    log::info!("Successfully removed registry block directive from Sandboxie.ini");
    Ok(format!(
        "Successfully updated Sandboxie.ini at: {}",
        sandbox_config_path
    ))
}

#[tauri::command]
pub async fn sandboxie_permit_plugins_folder() -> Result<String, String> {
    // Locate the Sandboxie.ini file
    let sandbox_ini = detect_sandboxie_ini().ok_or_else(|| {
        log::error!("Sandboxie.ini not found");
        "Sandboxie.ini not found".to_string()
    })?;
    let sandbox_config_path = sandbox_ini.to_string_lossy().to_string();

    // Get the Lethe plugins folder path
    let plugins_path = get_lethe_plugins_folder_location().map_err(|e| {
        log::error!("Failed to get plugins folder: {}", e);
        format!("Failed to get plugins folder: {}", e)
    })?;
    let plugins_path_str = plugins_path.to_string_lossy().to_string();

    // Construct the directive
    let directive = format!("OpenFilePath={}", plugins_path_str);

    // Add the directive
    add_directive(&sandbox_config_path, &directive).map_err(|e| {
        log::error!("Failed to update Sandboxie.ini: {}", e);
        format!("Failed to update Sandboxie.ini: {}", e)
    })?;

    log::info!("Successfully added directive to Sandboxie.ini");
    Ok(format!(
        "Successfully updated Sandboxie.ini at: {}",
        sandbox_config_path
    ))
}

#[tauri::command]
pub async fn sandboxie_permit_game_folder() -> Result<String, String> {
    // Locate the Sandboxie.ini file
    let sandbox_ini = detect_sandboxie_ini().ok_or_else(|| {
        log::error!("Sandboxie.ini not found");
        "Sandboxie.ini not found".to_string()
    })?;
    let sandbox_config_path = sandbox_ini.to_string_lossy().to_string();

    // Get the Lethe plugins folder path
    let plugins_path = get_lethe_limbus_folder_location().map_err(|e| {
        log::error!("Failed to get plugins folder: {}", e);
        format!("Failed to get plugins folder: {}", e)
    })?;
    let plugins_path_str = plugins_path.to_string_lossy().to_string();

    // Construct the directive
    let directive = format!("OpenFilePath={}", plugins_path_str);

    // Add the directive
    add_directive(&sandbox_config_path, &directive).map_err(|e| {
        log::error!("Failed to update Sandboxie.ini: {}", e);
        format!("Failed to update Sandboxie.ini: {}", e)
    })?;

    log::info!("Successfully added directive to Sandboxie.ini");
    Ok(format!(
        "Successfully updated Sandboxie.ini at: {}",
        sandbox_config_path
    ))
}

#[tauri::command]
pub async fn sandboxie_revoke_game_folder() -> Result<String, String> {
    // Locate the Sandboxie.ini file
    let sandbox_ini = detect_sandboxie_ini().ok_or_else(|| {
        log::error!("Sandboxie.ini not found");
        "Sandboxie.ini not found".to_string()
    })?;
    let sandbox_config_path = sandbox_ini.to_string_lossy().to_string();

    // Get the Lethe plugins folder path
    let plugins_path = get_lethe_limbus_folder_location().map_err(|e| {
        log::error!("Failed to get plugins folder: {}", e);
        format!("Failed to get plugins folder: {}", e)
    })?;
    let plugins_path_str = plugins_path.to_string_lossy().to_string();

    // Construct the directive
    let directive = format!("OpenFilePath={}", plugins_path_str);

    // Remove the directive
    remove_directive(&sandbox_config_path, &directive).map_err(|e| {
        log::error!("Failed to update Sandboxie.ini: {}", e);
        format!("Failed to update Sandboxie.ini: {}", e)
    })?;

    log::info!("Successfully removed directive from Sandboxie.ini");
    Ok(format!(
        "Successfully updated Sandboxie.ini at: {}",
        sandbox_config_path
    ))
}

#[tauri::command]
pub async fn sandboxie_block_cache_folders() -> Result<String, String> {
    // Locate the Sandboxie.ini file
    let sandbox_ini = detect_sandboxie_ini().ok_or_else(|| {
        log::error!("Sandboxie.ini not found");
        "Sandboxie.ini not found".to_string()
    })?;
    let sandbox_config_path = sandbox_ini.to_string_lossy().to_string();

    // Get the cache directories
    let cache_dirs = get_cache_directories().map_err(|e| {
        log::error!("Failed to get cache directories: {}", e);
        format!("Failed to get cache directories: {}", e)
    })?;

    // Construct the directives for blocking cache folders
    let directives = vec![
        format!("WriteFilePath={}", cache_dirs.local_app_data.display()),
        format!("WriteFilePath={}", cache_dirs.roaming.display()),
        format!("WriteFilePath={}", cache_dirs.local_low.display()),
    ];

    // Add each directive
    for directive in directives {
        add_directive(&sandbox_config_path, &directive).map_err(|e| {
            log::error!("Failed to add directive to Sandboxie.ini: {}", e);
            format!("Failed to add directive to Sandboxie.ini: {}", e)
        })?;
    }

    log::info!("Successfully added directives to Sandboxie.ini");
    Ok(format!(
        "Successfully updated Sandboxie.ini at: {}",
        sandbox_config_path
    ))
}

#[tauri::command]
pub async fn sandboxie_revoke_plugins_folder() -> Result<String, String> {
    // Locate the Sandboxie.ini file
    let sandbox_ini = detect_sandboxie_ini().ok_or_else(|| {
        log::error!("Sandboxie.ini not found");
        "Sandboxie.ini not found".to_string()
    })?;
    let sandbox_config_path = sandbox_ini.to_string_lossy().to_string();

    // Get the Lethe plugins folder path
    let plugins_path = get_lethe_plugins_folder_location().map_err(|e| {
        log::error!("Failed to get plugins folder: {}", e);
        format!("Failed to get plugins folder: {}", e)
    })?;
    let plugins_path_str = plugins_path.to_string_lossy().to_string();

    // Construct the directive
    let directive = format!("OpenFilePath={}", plugins_path_str);

    // Remove the directive
    remove_directive(&sandbox_config_path, &directive).map_err(|e| {
        log::error!("Failed to update Sandboxie.ini: {}", e);
        format!("Failed to update Sandboxie.ini: {}", e)
    })?;

    log::info!("Successfully removed directive from Sandboxie.ini");
    Ok(format!(
        "Successfully updated Sandboxie.ini at: {}",
        sandbox_config_path
    ))
}

#[tauri::command]
pub async fn sandboxie_unblock_cache_folders() -> Result<String, String> {
    // Locate the Sandboxie.ini file
    let sandbox_ini = detect_sandboxie_ini().ok_or_else(|| {
        log::error!("Sandboxie.ini not found");
        "Sandboxie.ini not found".to_string()
    })?;
    let sandbox_config_path = sandbox_ini.to_string_lossy().to_string();

    // Get the cache directories
    let cache_dirs = get_cache_directories().map_err(|e| {
        log::error!("Failed to get cache directories: {}", e);
        format!("Failed to get cache directories: {}", e)
    })?;

    // Construct the directives for blocking cache folders
    let directives = vec![
        format!("WriteFilePath={}", cache_dirs.local_app_data.display()),
        format!("WriteFilePath={}", cache_dirs.roaming.display()),
        format!("WriteFilePath={}", cache_dirs.local_low.display()),
    ];

    // Remove each directive
    for directive in directives {
        remove_directive(&sandbox_config_path, &directive).map_err(|e| {
            log::error!("Failed to remove directive from Sandboxie.ini: {}", e);
            format!("Failed to remove directive from Sandboxie.ini: {}", e)
        })?;
    }

    log::info!("Successfully removed directives from Sandboxie.ini");
    Ok(format!(
        "Successfully updated Sandboxie.ini at: {}",
        sandbox_config_path
    ))
}
