import {User} from "./user.ts";

export type Beatmapper = {
    id: number,
    beatmap_count: number,
    play_count: number,
    play_count_daily: number,
    download_count: number,
    upvote_count: number,
    downvote_count: number,
    vote_diff: number,
    vote_avg: number,
    upvote_avg: number,
    downvote_avg: number,
    download_avg: number,
    score: string,
    rating: string,
    is_curated: boolean,
    is_hot: boolean,
    last_beatmap_at: string,
    curated_qualified_at: string,
    user: User
}