<script setup lang="ts">

import {provide, ref} from "vue";

type Toast = {
  content: string,
  type: string,
  duration: number,
};

const toasts = ref<Toast[]>([]);

provide("toast_container", this);

function add(toast: Toast): void {
  console.log("Adding new toast:",toast);
  toasts.value?.push(toast);
}

defineExpose({"add": add});

</script>

<template>
<div class="toast toast-end">
  <div class="alert" v-for="toast of toasts" :class="'alert-' + toast.type">
    <span>{{toast.content}}</span>
  </div>
</div>
</template>

<style scoped>

</style>