use serde::Serialize;
use tauri::{ipc::Channel, AppHandle};

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
