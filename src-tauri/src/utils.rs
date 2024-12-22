use std::{
    env,
    path::{Path, PathBuf},
};

use serde_json::Value;
use tauri::Wry;
use tauri_plugin_store::Store;

/// Utility function to detect the Windows directory based on the `windir` environment variable.
pub fn detect_windows_dir() -> Option<String> {
    if let Ok(windir) = env::var("windir") {
        let windows_dir = Path::new(&windir);
        if windows_dir.exists() && windows_dir.is_dir() {
            return Some(windir);
        }
    }
    None
}

/// Utility function to detect the Sandboxie.ini file location.
/// Returns the path to the Sandboxie.ini file if it exists.
pub fn detect_sandboxie_ini() -> Option<PathBuf> {
    if let Some(windows_dir) = detect_windows_dir() {
        let sandboxie_ini_path = Path::new(&windows_dir).join("Sandboxie.ini");
        if sandboxie_ini_path.exists() && sandboxie_ini_path.is_file() {
            return Some(sandboxie_ini_path);
        }
    }
    None
}

/// Utility function that extracts `value` from the tauri Store
pub fn extract_value<T>(store: &Store<Wry>, key: &str, default: T) -> T
where
    T: serde::de::DeserializeOwned,
{
    match store.get(key) {
        Some(Value::Object(map)) => match map.get("value") {
            Some(value) => {
                // Attempt to deserialize the value to the expected type T
                match serde_json::from_value(value.clone()) {
                    Ok(val) => val,
                    Err(_) => {
                        println!(
                            "Failed to deserialize {} value, falling back to default.",
                            key
                        );
                        default
                    }
                }
            }
            None => {
                println!("{} value not found, falling back to default.", key);
                default
            }
        },
        _ => {
            println!(
                "{} key not found or not an object, falling back to default.",
                key
            );
            default
        }
    }
}
