use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistStruct {
    data_string: Vec<PlaylistEntry>,
    #[serde(rename = "SelectedIconIndex")]
    selected_icon_index: u8,
    #[serde(rename = "SelectedTexture")]
    selected_texture: u8,
    name_playlist: String,
    description: String,
    gradient_top: String,
    gradient_down: String,
    color_title: String,
    color_texture: String,
    creation_date: u64
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistEntry {
    hash: String,
    name: String,
    author: String,
    beatmapper: String,
    difficulty: u8,
    track_duration: f32,
    added_time: u64,
}