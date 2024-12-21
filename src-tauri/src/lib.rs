use commands::download::{download_and_extract_bepinex, download_and_install_lethe};
use commands::file_utils::{check_lethe_limbus_up_to_date, clone_folder_to_game, open_game_folder};
use commands::patch::patch_limbus;
use commands::steam::steam_limbus_location;
use std::path::PathBuf;
use std::{env, fs};
use tauri_plugin_deep_link::DeepLinkExt;
use tauri_plugin_store::StoreExt;
use tokio::task;
use utils::extract_value;
mod commands;
mod utils;

fn set_current_dir_to_container_appdata() {
    let local_appdata = env::var("LOCALAPPDATA").expect("Failed to get LOCALAPPDATA");
    let target_dir: PathBuf = PathBuf::from(&local_appdata).join("Packages/zweilauncher/AC");
    fs::create_dir_all(&target_dir).expect("Failed to create target directory");
    env::set_current_dir(&target_dir).expect("Failed to set current directory");
    log::info!("Current directory set to: {}", target_dir.display());
}

fn set_current_dir_to_exe() {
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let exe_dir = exe_path.parent().expect("Failed to get parent directory");

    env::set_current_dir(exe_dir)
        .expect("Failed to set current directory to executable's location");

    log::info!("Current directory set to: {}", exe_dir.display());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    set_current_dir_to_exe();

    let mut builder = tauri::Builder::default().plugin(
        tauri_plugin_log::Builder::new()
            .target(tauri_plugin_log::Target::new(
                tauri_plugin_log::TargetKind::LogDir {
                    file_name: Some("logs".to_string()),
                },
            ))
            .build(),
    );
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|_app, argv, _cwd| {
          log::info!("a new app instance was opened with {argv:?} and the deep link event was already triggered");
        }));
    }

    builder
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_deep_link::init())
        .invoke_handler(tauri::generate_handler![
            steam_limbus_location,
            download_and_extract_bepinex,
            download_and_install_lethe,
            patch_limbus,
            open_game_folder,
            clone_folder_to_game,
            check_lethe_limbus_up_to_date
        ])
        .setup(|app| {
            // Create a new store or load the existing one
            // this also put the store in the app's resource table
            // so your following calls `store` calls (from both rust and js)
            // will reuse the same store
            let store = app.store("store.json")?;

            let app_handle = app.handle().clone();
            #[cfg(any(windows, target_os = "linux"))]
            {
                app.deep_link().register_all()?;
            }

            app.deep_link().on_open_url(move |event| {
                let handle_clone = app_handle.clone();
                let launch_args: String = extract_value(&store, "launchArgs", "".to_string());
                let is_sandbox: bool = extract_value(&store, "isSandbox", false);
                let sandbox_path: String = extract_value(&store, "sandboxPath", "".to_string());

                let urls = event.urls();
                let owned_urls: Vec<_> = urls.into_iter().collect(); // Due to rust ownership system we must fully own every url here

                if let Some(first_url) = owned_urls.first() {
                    if let Some(token) = first_url.query() {
                        let launch_args_clone = launch_args.clone();
                        let token_clone = token.to_string(); // Another owned string conversion here
                        let handle_clone = handle_clone.clone(); // Clone the handle again for the async block

                        // Delegate launch game to tokio to prevent blocking the main thread
                        task::spawn(async move {
                            commands::game::launch_game(
                                handle_clone,
                                launch_args_clone,
                                token_clone,
                                is_sandbox,
                                sandbox_path,
                            )
                            .await;
                        });
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
