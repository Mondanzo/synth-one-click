use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use tauri::{command, ipc::Channel, AppHandle};

#[derive(Clone, Serialize)]
enum DownloadType {
    Beatmap,
    Stage,
    Avatar,
    Playlist,
}

#[derive(Clone, Serialize)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "event",
    content = "data"
)]
enum DownloadEvent {
    Started {
        r#type: DownloadType,
        id: usize,
        content_length: u16,
    },
    Progress {
        id: usize,
        content_length: u16,
    },
    Completed {
        id: usize,
    },
}

fn download(app: AppHandle, url: String, folder: String, on_event: Channel<DownloadEvent>) {
}


#[derive(Deserialize, Serialize, Debug)]
pub struct PlaylistStruct {
    dataString: Vec<PlaylistEntry>,
    SelectedIconIndex: u8,
    SelectedTexture: u8,
    namePlaylist: String,
    description: String,
    gradientTop: String,
    gradientDown: String,
    colorTitle: String,
    colorTexture: String,
    creationDate: u64
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PlaylistEntry {
    hash: String,
    name: String,
    author: String,
    beatmapper: String,
    difficulty: u8,
    trackDuration: f32,
    addedTime: u64,
}


#[command]
pub fn get_playlist(playlist_file: String) -> Option<PlaylistStruct> {
    let path = Path::new(&playlist_file);
    match path.try_exists() {
        Err(_) => None,
        Ok(val) => {
            match val {
                false => None,
                true => {
                    println!("Found Playlist file.");
                    let data: Result<PlaylistStruct, Error> = serde_json::from_reader(fs::File::open(playlist_file).unwrap());

                    println!("{:?}", data);

                    match data {
                        Err(_) => None,
                        Ok(playlist) => Some(playlist)
                    }
                }
            }
        }
    }
}