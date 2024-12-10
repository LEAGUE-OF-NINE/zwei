use serde_json::Value;
use tauri::Wry;
use tauri_plugin_store::Store;

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
