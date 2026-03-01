use crate::AppState;
use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};
use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::SystemTime;
use tauri::ipc::Channel;
use tauri::{Manager, Wry};
use tauri_plugin_store::{Store, StoreExt};
use tokio::task::JoinHandle;

#[derive(Clone, Serialize)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "event",
    content = "data"
)]
pub enum CacheEvent {
    NewTotal { total: u32 },
    CacheUpdate {},
    Completed {},
}

#[derive(Clone, Debug)]
pub struct BeatmapsCache {
    beatmaps_path: String,
    cache_path: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde()]
struct CachedData {
    last_cache: SystemTime,
    hash: String,
}

impl BeatmapsCache {
    pub(crate) fn new(beatmaps_path: String, cache_path: String) -> BeatmapsCache {
        BeatmapsCache {
            beatmaps_path,
            cache_path,
        }
    }
}

fn check_if_synth_file(fo: &DirEntry) -> bool {
    fo.file_type().unwrap().is_file() && fo.path().extension().unwrap().eq("synth")
}

async fn add_entry_to_cache(
    store: Arc<Store<Wry>>,
    path: PathBuf,
    cache_key: String,
    cache_channel: Channel<CacheEvent>,
) {
    let entry = create_cache_entry_for(path.to_str().unwrap()).await;
    if entry.is_some() {
        store.set(cache_key, serde_json::to_value(entry.clone()).unwrap());
    }
    cache_channel.send(CacheEvent::CacheUpdate {});
}

#[tauri::command]
pub async fn bmc_regenerate_cache(
    app: tauri::AppHandle,
    on_cache_channel: Channel<CacheEvent>,
) -> Result<(), ()> {
    println!("Starting cache refreshing...");
    let bmc_o = app.state::<AppState>().beatmaps_cache.clone();
    if bmc_o.is_none() {
        return Err(());
    }
    let bmc = bmc_o.unwrap();

    let store = app.store(bmc.cache_path.clone()).unwrap();
    let beatmaps_path = bmc.beatmaps_path.clone();
    let mut total: u32 = 0;
    store.clear();
    println!("Cleared store.");

    let mut futures: Vec<JoinHandle<()>> = Vec::new();

    println!("Iterating through files for folder {:?}", bmc.beatmaps_path);
    for file in fs::read_dir(beatmaps_path.clone()).unwrap().into_iter() {
        let fo = file.unwrap();
        if check_if_synth_file(&fo) {
            total += 1;
            // Synth File we can create cache entry for!
            let path = fo.path().to_path_buf();

            let cache_key = path
                .strip_prefix(beatmaps_path.clone())
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            let future = tokio::spawn(add_entry_to_cache(
                store.clone(),
                path,
                cache_key,
                on_cache_channel.clone(),
            ));
            futures.push(future);
            on_cache_channel.send(CacheEvent::NewTotal { total });
        }
    }

    for future in futures.into_iter() {
        future.await;
    }

    on_cache_channel.send(CacheEvent::Completed {});

    Ok(())
}

fn get_hash_from_file(file: &str) -> Option<String> {
    let file = fs::File::open(file).unwrap();
    let fr = zip::ZipArchive::new(file);

    let mut zip = fr.unwrap();
    let meta_file = zip.by_path("synthriderz.meta.json");

    match meta_file {
        Err(_) => None,
        Ok(value) => match value.is_file() {
            false => None,
            true => {
                let data: Result<Value, Error> = serde_json::from_reader(value);
                match data {
                    Err(_) => None,
                    Ok(value) => Some(value.get("hash").unwrap().as_str().unwrap().to_string()),
                }
            }
        },
    }
}

async fn create_cache_entry_for(file: &str) -> Option<CachedData> {
    let hash = get_hash_from_file(file);

    hash.map(|hash| CachedData {
        hash,
        last_cache: SystemTime::now(),
    })
}

#[tauri::command]
pub async fn bmc_exists(
    file: &str,
    hash: &str,
    app: tauri::AppHandle,
) -> Result<bool, ()> {
    let path = Path::new(file);
    let bmc_o = app.state::<AppState>().beatmaps_cache.clone();
    if bmc_o.is_none() {
        return Ok(false);
    }

    let bmc = bmc_o.unwrap();

    let file_name = path.strip_prefix(bmc.beatmaps_path.clone());
    if fs::exists(file).is_err() {
        return Ok(false);
    }

    if file_name.is_err() {
        return Ok(false);
    }

    let store = app.store(bmc.cache_path.clone()).unwrap();
    let metadata = fs::metadata(file).unwrap();

    let cache_key = file_name.unwrap().to_str().unwrap();

    let cache_data: Result<CachedData, Error> =
        serde_json::from_value(store.get(cache_key).unwrap());

    match cache_data {
        Err(_) => {
            // No entry found. Try make cache entry.
            let entry = create_cache_entry_for(file).await;
            match entry {
                Some(entry) => {
                    store.set(cache_key, serde_json::to_value(entry.clone()).unwrap());
                    Ok(entry.hash.eq(hash))
                }
                None => Ok(false),
            }
        }
        Ok(value) => {
            match value.last_cache < metadata.modified().unwrap() {
                true => {
                    // Modified since last cache.
                    // Make new cache entry.
                    let entry = create_cache_entry_for(file).await;
                    match entry {
                        Some(entry) => {
                            store.set(cache_key, serde_json::to_value(entry.clone()).unwrap());
                            Ok(entry.hash.eq(hash))
                        }
                        None => Ok(false),
                    }
                }
                false => Ok(hash.eq(&value.hash)),
            }
        }
    }
}


#[tauri::command]
pub async fn bmc_count(app: tauri::AppHandle) -> u32 {
    let app_state = app.state::<AppState>();
    let cache_settings = app_state.beatmaps_cache.clone();
    match cache_settings {
        None => 0,
        Some(opts) => {
            let store = app.store(opts.cache_path).unwrap();
            let file_path = Path::new(opts.beatmaps_path.as_str()).join("SynthridersUC").join("CustomSongs");

            let mut count: u32 = 0;
            for _ in store.keys() {
                count += 1;
            }

            count
        }
    }
}

#[tauri::command]
pub async fn bmc_files(app: tauri::AppHandle) -> u32 {
    let app_state = app.state::<AppState>();
    let cache_settings = app_state.beatmaps_cache.clone();
    match cache_settings {
        None => 0,
        Some(opts) => {
            let file_path = Path::new(opts.beatmaps_path.as_str());

            let mut count: u32 = 0;
            for file in fs::read_dir(file_path).unwrap() {
                if check_if_synth_file(&file.unwrap()) {
                    count += 1;
                }
            }

            count
        }
    }
}