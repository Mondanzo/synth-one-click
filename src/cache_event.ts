import {BaseEvent} from "./plugins/event-bus.ts";

export type CacheNewTotalEvent = BaseEvent<"newTotal", {
    total: number
}>
export type CacheUpdateEvent = BaseEvent<"cacheUpdate", never>
export type CacheCompletedEvent = BaseEvent<"completed", never>

export type CacheEvent = CacheNewTotalEvent | CacheUpdateEvent | CacheCompletedEvent;