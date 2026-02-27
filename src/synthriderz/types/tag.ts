import {Visiblity} from "./generic/visiblity.ts";

export type Tag = {
    id: number,
    name: string,
    slug: string,
    description?: string,
    visibility: Visiblity,
    category?: string,
    created_at: string,
}