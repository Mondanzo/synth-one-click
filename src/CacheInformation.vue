<script setup lang="ts">

import {Channel, invoke} from "@tauri-apps/api/core";
import {inject, ref} from "vue";
import {CacheEvent} from "./cache_event.ts";
import {EventBusKey} from "./plugins/event-bus.ts";

const running = ref(false);
const cacheTotal = ref(0);
const filesTotal = ref(0);

const eventBus = inject(EventBusKey)!;

function obtain_count() {
  invoke<number>("bmc_count").then(val => {
    cacheTotal.value = val;
  });

  invoke<number>("bmc_files").then(val => {
    filesTotal.value = val;
  });
}

obtain_count();

async function refreshCache() {
  const onCacheUpdate = new Channel<CacheEvent>((event) => {
    eventBus.emit(event);
  });

  running.value = true;
  await invoke("bmc_regenerate_cache", {onCacheChannel: onCacheUpdate});
  running.value = false;
  obtain_count();
}
</script>

<template>
  <div class="flex items-center gap-2">
    <p>Currently Cached: {{ cacheTotal }}<br/>Discovered files: {{filesTotal}}</p>
    <button @click="() => refreshCache()" :disabled="running" class="btn btn-info btn-outline w-fit"
            :class="{'disabled': running}">Refresh Cache.
    </button>
  </div>
</template>

<style scoped>

</style>