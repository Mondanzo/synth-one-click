<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core";
import {computed, inject, ref, useTemplateRef} from "vue";
import {configKey} from "./ConfigUtils";
import {open as fileOpener} from "@tauri-apps/plugin-dialog";
import ToastContainer from "./ToastContainer.vue";
import {ExclamationCircleIcon} from "@heroicons/vue/24/solid";
import {isRegistered} from "@tauri-apps/plugin-deep-link";
import CacheProgress from "./CacheProgress.vue";
import CacheInformation from "./CacheInformation.vue";

const config = inject(configKey);

const synthFolder = ref("");
const savedFolder = ref("");

const existingMode = ref(0);
const savedExistingMode = ref(0);

const debugInfo = ref(false);
const savedDebugInfo = ref(false);

const toastContainer = useTemplateRef<InstanceType<typeof ToastContainer>>("toastContainer");

const isChangedSynthFolder = computed(() => synthFolder.value !== savedFolder.value);
const isChangedExistingMode = computed(() => existingMode.value !== savedExistingMode.value);
const isChangedDebugInfo = computed(() => debugInfo.value !== savedDebugInfo.value);

config?.get<string>("synth_folder").then(val => {
  if (val) {
    synthFolder.value = val;
    savedFolder.value = val;
  }
});

config?.get<number>("existing_mode").then(val => {
  if (val) {
    existingMode.value = val;
    savedExistingMode.value = val;
  }
});

config?.get<boolean>("debug_info").then(val => {
  if (val) {
    debugInfo.value = val;
    savedDebugInfo.value = val;
  }
});

async function save() {
  console.log("Saving synth folder:", synthFolder.value);
  await invoke("set_synth_folder", {path: synthFolder.value});
  savedFolder.value = await config?.get<string>("synth_folder") ?? "";
  await config?.set("existing_mode", existingMode.value);
  savedExistingMode.value = existingMode.value;

  await config?.set("debug_info", debugInfo.value);
  savedDebugInfo.value = debugInfo.value;
}


async function discoverSyntriders() {
  const result = await invoke<string | null>("discover_synthriders");
  console.log(result);
  if (result) {
    synthFolder.value = result;
    toastContainer.value!.add({content: "Successfully auto-discovered.", duration: 5000, type: "success"})
  } else {
    toastContainer.value!.add({content: "Faild to auto-discover Synth Riders folder.", duration: 5000, type: "error"})
  }
}

const isRegisteringUrl = ref(false);

async function registerUrl() {
  isRegisteringUrl.value = true;
  try {
    await invoke("register_url");
    urlRegistered.value = await isRegistered("synthriderz");
  } catch (error) {

  }
  isRegisteringUrl.value = false;
}


const urlRegistered = ref();

isRegistered('synthriderz').then(value => urlRegistered.value = value);


async function getDirectory() {
  const filePath = await fileOpener({
    multiple: false,
    directory: true
  });
  if (filePath) {
    synthFolder.value = filePath;
  }
}
</script>

<template>
  <main class="m-4 flex flex-col gap-4">
    <h1 class="text-primary text-3xl font-bold">Synthriderz One Click Installer</h1>
    <div class="alert alert-error flex justify-between" v-show="!urlRegistered">
      <span>One Click Installer is not set up properly.</span>
      <button @click="registerUrl" class="btn btn-neutral" :disabled="isRegisteringUrl">Click here to fix <span
          v-if="isRegisteringUrl" class="loading loading-spinner"/>
      </button>
    </div>
    <form @submit.prevent="save">
      <fieldset class="fieldset border rounded-box p-4 flex flex-col gap-4">
        <legend class="fieldset-label text-lg px-2">Config</legend>
        <div class="w-full">
          <div class="flex items-start gap-2 w-full">
            <ExclamationCircleIcon class="h-5 animate-reveal" :class="{'invisible': !isChangedSynthFolder}"/>
            <div class="flex flex-col w-full">
              <label class="label mb-2">Synth Riders Folder</label>
              <div class="join grow">
                <input class="input join-item w-full" type="text" v-model="synthFolder" @dblclick="getDirectory"/>
                <button class="btn btn-secondary join-item" @click="discoverSyntriders">Auto-Discover</button>
              </div>
            </div>
          </div>
        </div>
        <div class="flex items-center gap-2">
          <ExclamationCircleIcon class="h-5 animate-reveal" :class="{'invisible': !isChangedExistingMode}"/>
          <label for="existing-mode">What to do on existing Beatmap?</label>
          <select class="select" name="existing-mode" id="existing-mode" v-model="existingMode">
            <option :value="0">Ask</option>
            <option :value="1">Skip</option>
            <option :value="2">Redownload</option>
          </select>
        </div>
        <div class="flex items-center gap-2">
          <ExclamationCircleIcon class="h-5 animate-reveal" :class="{'invisible': !isChangedDebugInfo}"/>
          <label for="debug-info">Show Debug Information?</label>
          <input type="checkbox" class="checkbox" name="debug-info" id="debug-info" v-model="debugInfo">
        </div>
        <button class="btn btn-primary w-fit" type="submit">Save</button>
      </fieldset>
      <fieldset class="fieldset border border-base-300 rounded p-4" v-if="savedDebugInfo">
        <legend class="text-neutral-200 px-2">Debug</legend>
        <CacheInformation />
      </fieldset>
    </form>
  </main>
  <toast-container class="z-10" ref="toastContainer"/>
  <cache-progress/>
</template>

<style>
@keyframes reveal {
  from {
    transform: translateX(100%) scale(0.8);
    opacity: 0;
  }

  to {
    transform: translateX(0) scale(1);
    opacity: 1;
  }
}

@keyframes hide {
  from {
    transform: translateX(0) scale(1);
    opacity: 1;
    visibility: visible;
  }

  to {
    transform: translateX(100%) scale(0.8);
    opacity: 0;
    visibility: hidden;
  }
}
.animate-reveal {
  animation: reveal 200ms cubic-bezier(0, 0.55, 0.45, 1);
  &.invisible {
    animation: hide 200ms cubic-bezier(0.55, 0, 1, 0.45);
  }
}
</style>