<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core";
import {computed, inject, ref, useTemplateRef} from "vue";
import {configKey} from "./ConfigUtils";
import {open as fileOpener} from "@tauri-apps/plugin-dialog";
import ToastContainer from "./ToastContainer.vue";
import {ExclamationTriangleIcon} from "@heroicons/vue/24/solid";
import {isRegistered} from "@tauri-apps/plugin-deep-link";

const config = inject(configKey);
const synthFolder = ref("");
const toastContainer = useTemplateRef<InstanceType<typeof ToastContainer>>("toastContainer");

config?.get<string>("synth_folder").then(val => {
  if (val) {
    synthFolder.value = val;
    savedFolder.value = val;
  }
});

const savedFolder = ref("");

const isChanged = computed(() => synthFolder.value !== savedFolder.value);

async function save() {
  console.log("Saving synth folder:", synthFolder.value);
  await config?.set("synth_folder", synthFolder.value);
  await config?.save();
  savedFolder.value = synthFolder.value;
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
    <h1 class="text-primary text-3xl font-bold">SynthOneClick</h1>
    <div class="alert alert-error flex justify-between" v-show="!urlRegistered">
      <span>One Click Installer is not set up properly.</span>
      <button @click="registerUrl" class="btn btn-neutral" :disabled="isRegisteringUrl">Click here to fix <span
          v-if="isRegisteringUrl" class="loading loading-spinner"/>
      </button>
    </div>
    <form @submit.prevent="save">
      <fieldset class="fieldset border rounded-box p-4">
        <legend class="fieldset-label text-lg px-2">Config</legend>
        <label class="label">Synth Riders Folder</label>
        <div class="flex items-center gap-2 w-full">
          <ExclamationTriangleIcon class="h-5" v-if="isChanged"/>
          <div class="join grow">
            <input class="input join-item w-full" type="text" v-model="synthFolder" @click="getDirectory"/>
            <button class="btn btn-secondary join-item" @click="discoverSyntriders">Auto-Discover</button>
          </div>
        </div>
        <button class="btn btn-primary w-fit" type="submit">Save</button>
      </fieldset>
    </form>
  </main>
  <toast-container ref="toastContainer"/>
</template>