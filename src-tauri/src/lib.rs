mod download;
mod beatmaps_cache;
mod discover_commands;

use std::path::Path;
use discover_commands::discover_synthriders;
use discover_commands::synth_id;
use tauri::{AppHandle, Manager};
use tauri_plugin_deep_link::DeepLinkExt;
use tauri_plugin_store::StoreExt;
use crate::beatmaps_cache::*;
use crate::download::get_playlist;

#[derive(Debug)]
struct AppState {
    beatmaps_cache: Option<BeatmapsCache>
}

#[tauri::command]
fn register_url(app_handle: AppHandle) -> bool {
    app_handle.deep_link().register("synthriderz").is_ok()
}

#[tauri::command]
fn set_synth_folder(app: AppHandle, path: &str) {
    let store = app.store("config.json").unwrap();
    store.set("synth_folder", path);

    app.manage(AppState {
        beatmaps_cache: Some(BeatmapsCache::new(Path::new(path).join("SynthridersUC").join("CustomSongs").to_str().unwrap().to_string(), String::from("beatmaps_cache")))
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let store = app.store("config.json")?;

            let value = store.get("synth_folder");

            let ok = app.manage(AppState {
                beatmaps_cache: match value {
                    None => None,
                    Some(field) => {
                        match field.is_string() {
                            false => {
                                println!("Field is not a string.");
                                None
                            },
                            true => {
                                let beatmaps_path = Path::new(field.as_str().unwrap()).join("SynthridersUC").join("CustomSongs");
                                println!("Setting Beatmaps Path to: {:?}", beatmaps_path);
                                Some(BeatmapsCache::new(beatmaps_path.to_str().unwrap().to_string(), String::from("beatmaps_cache"))) }
                        }
                    }
                }
            });

            let state = app.state::<AppState>();

            println!("Obtained state: {:?}", state);

            println!("State set successfully? {:?}", ok);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_synth_folder,
            discover_synthriders,
            synth_id,
            register_url,
            bmc_regenerate_cache,
            bmc_exists,
            bmc_count,
            bmc_files,
            get_playlist
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}