import {User} from "./user.ts";
import {Difficulty} from "./generic/difficulty.ts";

export type Playlist = {
    id: number,
    name: string,
    description: string,
    gradientTop: string,
    gradientDown: string,
    colorTitle: string,
    colorTexture: string,
    texture: number,
    icon: number,
    filename_original: string,
    cover_version: number,
    download_count: number,
    upvote_count: number,
    downvote_count: number,
    vote_diff: number,
    score: string,
    rating: string,
    published: boolean,
    published_at: string,
    created_at: string,
    updated_at: string,
    version: number,
    auto_playlist_id: null,
    user: User,
    download_url: string,
    cover_url: string
}

export type PlaylistFileEntry = {
    addedTime: number,
    author: string,
    beatmapper: string,
    difficulty: Difficulty,
    hash: string,
    name: string,
    trackDuration: number
}

export type PlaylistFile = {
    SelectedIconIndex: number
    SelectedTexture: number
    colorTexture: string,
    colorTitle: string,
    creationDate: number,
    dataString: PlaylistFileEntry[],
    description: string
    gradientDown: string,
    gradientTop: string
    namePlaylist: string
}