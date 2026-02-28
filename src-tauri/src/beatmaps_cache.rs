use serde::{Deserialize, Serialize};
use std::{fs};
use std::fs::metadata;
use std::path::Path;
use std::time::SystemTime;
use serde_json::{Error, Value};
use tauri_plugin_store::{StoreExt};

#[derive(Clone)]
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

#[tauri::command]
pub async fn bmc_regenerate_cache(
    app: tauri::AppHandle,
    cache: tauri::State<'_, BeatmapsCache>,
) -> Result<(), ()> {
    let store = app.store(cache.cache_path.clone()).unwrap();
    store.clear();
    for file in fs::read_dir(cache.beatmaps_path.clone()).unwrap() {
        let fo = file.unwrap();
        if fo.file_type().unwrap().is_file() {
            if fo.path().extension().unwrap().eq("synth") {
                // Synth File we can create cache entry for!
                let path = fo.path();

                let cache_key = path.strip_prefix(cache.beatmaps_path.clone());

                let entry = create_cache_entry_for(path.to_str().unwrap()).await;
                store.set(cache_key.unwrap().to_str().unwrap(), serde_json::to_value(entry.clone()).unwrap());
            }
        }
    }

    Ok(())
}


fn get_hash_from_file(file: &str) -> Option<String> {
    let file = fs::File::open(file).unwrap();
    let fr = zip::ZipArchive::new(file);

    let mut zip = fr.unwrap();
    let metaFile = zip.by_path("synthriderz.meta.json");

    match metaFile {
        Err(_) => None,
        Ok(value) => {
            match value.is_file() {
                false => None,
                true => {
                    let data: Result<Value, Error> = serde_json::from_reader(value);
                    match data {
                        Err(_) => None,
                        Ok(value) => {
                            Some(value.get("hash").unwrap().as_str().unwrap().to_string())
                        }
                    }
                }
            }
        }
    }
}


async fn create_cache_entry_for(file: &str) -> Option<CachedData> {
    let hash = get_hash_from_file(file);

    match hash {
        None => None,
        Some(hash) => {
            Some(CachedData {
                hash,
                last_cache: SystemTime::now()
            })
        }
    }
}


#[tauri::command]
pub async fn bmc_exists(
    file: &str,
    hash: &str,
    app: tauri::AppHandle,
    cache: tauri::State<'_, BeatmapsCache>,
) -> Result<bool, ()> {
    let path = Path::new(file);
    let fileName = path.strip_prefix(cache.beatmaps_path.clone());
    if !fs::exists(file).is_ok() {
        return Ok(false);
    }

    if fileName.is_err() {
        return Ok(false);
    }

    let store = app.store(cache.cache_path.clone()).unwrap();
    let metadata = fs::metadata(file).unwrap();

    let cache_key = fileName.unwrap().to_str().unwrap();

    let cache_data: Result<CachedData, Error> = serde_json::from_value(store.get(cache_key).unwrap());

    match cache_data {
        Err(_) => {
            // No entry found. Try make cache entry.
            let entry = create_cache_entry_for(file).await;
            match entry {
                Some(entry) => {
                    store.set(cache_key, serde_json::to_value(entry.clone()).unwrap());
                    Ok(entry.hash.eq(hash))
                }
                None => Ok(false)
            }
        },
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
                        None => Ok(false)
                    }
                },
                false => {
                    Ok(hash.eq(&value.hash))
                }
            }
        }
    }
}