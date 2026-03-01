<script setup lang="ts">
import {inject, onMounted, ref} from "vue";
import {CacheNewTotalEvent, CacheUpdateEvent, CacheCompletedEvent} from "./cache_event.ts";
import {addEventListener, EventBusKey} from "./plugins/event-bus.ts";


const total = ref(0);
const current = ref(0);
const completed = ref(true);

const eventBus = inject(EventBusKey)!;

eventBus.addEventListener<CacheNewTotalEvent>("newTotal", (event) => {
  total.value = event.data.total;
  completed.value = false;
});

eventBus.addEventListener<CacheUpdateEvent>("cacheUpdate", (_) => {
  current.value++;
});

eventBus.addEventListener<CacheCompletedEvent>("completed", (_) => {
  completed.value = true;
});
</script>

<template>
  <div v-if="!completed"
       class="cache-progress-notice absolute bottom-0 alert alert-info alert-soft w-full flex flex-col">
    <div class="flex justify-center items-center w-full">
      <span>Updating caches {{ current }} / {{ total }} completed.</span>
    </div>
    <progress class="progress progress-info" :max="total" :value="current"></progress>
  </div>
</template>