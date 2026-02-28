use std::fs;
use std::path::Path;

#[cfg(windows)]
use windows_registry::*;

const SYNTH_RIDERS_APP: &str = "885000";

#[tauri::command]
pub fn synth_id() -> &'static str {
    SYNTH_RIDERS_APP
}

#[cfg(windows)]
pub fn discover_steam() -> Option<String> {
    println!("Discovering Steam");
    let collection = CURRENT_USER.open("SOFTWARE\\Valve\\Steam").unwrap();
    let synth_key = collection.get_string("SteamPath");
    synth_key.ok()
}

#[cfg(not(windows))]
fn discover_steam() -> Option<String> {
    None
}

#[tauri::command]
pub fn discover_synthriders() -> Option<String> {
    let steam_path = discover_steam();
    println!("Steam Result: {:?}", steam_path);
    match steam_path {
        None => None,
        Some(steam_path) => {
            println!("Discovered Steam Path at: {}", steam_path);
            let path = Path::new(&steam_path);
            let library_file = path.join("config/libraryfolders.vdf");
            match library_file.is_file() {
                false => None,
                true => {
                    println!("Found libraryfolders.vdf");
                    let data = fs::read_to_string(library_file).unwrap();
                    let vdf_parser = keyvalues_parser::parse(data.as_str()).unwrap();
                    let libs = vdf_parser.value.get_obj()?.clone().into_vdfs();

                    for lib in libs {
                        let obj = lib.value.get_obj().unwrap();
                        let has_app = obj
                            .get("apps")
                            .unwrap()
                            .first()
                            .unwrap()
                            .get_obj()
                            .unwrap()
                            .contains_key(SYNTH_RIDERS_APP);
                        if has_app {
                            let library_path =
                                obj.get("path").unwrap().first().unwrap().get_str().unwrap();
                            let base_path = Path::new(library_path);
                            let synth_folder = base_path
                                .join("steamapps")
                                .join("common")
                                .join("SynthRiders");
                            return match synth_folder.is_dir() {
                                false => None,
                                true => Some(synth_folder.to_str().unwrap().to_string()),
                            };
                        }
                    }

                    None
                }
            }
        }
    }
}