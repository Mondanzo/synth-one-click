<script setup lang="ts">
import {getCurrent} from "@tauri-apps/plugin-deep-link";
import {defineAsyncComponent, shallowRef} from "vue";

const component = shallowRef();

getCurrent().then(current => {
  if(current) {
    component.value = defineAsyncComponent(() => import("./OneClick.vue"))
  } else {
    component.value = defineAsyncComponent(() => import("./AppConfiguration.vue"))
  }
});
</script>

<template>
  <Suspense>
    <component v-if="component" :is="component"/>
  </Suspense>
</template>