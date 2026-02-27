import {User} from "./user.ts";

export type Avatar = {
    id: number,
    cover_version: number,
    name: string,
    description?: string,
    youtube_url?: string,
    filename_original: string,
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
    user: User,
    download_url: string,
    cover_url: string
}