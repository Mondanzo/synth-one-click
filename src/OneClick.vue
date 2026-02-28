<script setup lang="ts">
import {getCurrent} from "@tauri-apps/plugin-deep-link";
import {fetch} from "@tauri-apps/plugin-http";
import {configKey} from "./ConfigUtils";
import {inject, ref} from "vue";
import {path} from "@tauri-apps/api";
import {download} from "@tauri-apps/plugin-upload";
import {getCurrentWindow} from "@tauri-apps/api/window";
import {Stage} from "./synthriderz/types/stage.ts";
import {Playlist} from "./synthriderz/types/playlist.ts";
import {Avatar} from "./synthriderz/types/avatar.ts";

const current = (await getCurrent())!.pop();

const url = new URL(current!);

const config = inject(configKey)!;

const baseSynthAPI = "https://synthriderz.com/";

const assetName = ref("Undefined");
const assetAuthor = ref("");
const cover = ref<string | null>(null);
const downloadProgress = ref(0);


enum DownloadTypes {
  Beatmap = "Beatmap",
  Stage = "Stage",
  Avatar = "Avatar",
  Playlist = "Playlist",
  Undefined = "Undefined"
}

let installingWhat: DownloadTypes = DownloadTypes.Undefined;
const targetFolder = new Map<DownloadTypes, string>([
  [DownloadTypes.Beatmap, "./SynthridersUC/CustomSongs"],
  [DownloadTypes.Stage, "./SynthridersUC/CustomStages"],
  [DownloadTypes.Avatar, "./SynthridersUC/Avatars"]
]);

const extractFilename = (contentDisposition: string) => {
  if (!contentDisposition) return null;

  // Split header into parts (e.g., ["attachment", "filename*=UTF-8''my%20file.pdf"])
  const parts = contentDisposition.split(';').map(part => part.trim());

  // Case 1: Handle filename* (UTF-8 encoded)
  const filenameStarPart = parts.find(part => part.startsWith('filename*='));
  if (filenameStarPart) {
    // Regex to extract charset and encoded filename (e.g., "UTF-8''my%20file.pdf")
    const match = filenameStarPart.match(/filename\*=([^']*)''(.*)/);
    if (match) {
      const encodedFilename = match[2];
      return decodeURIComponent(encodedFilename); // Decode URL-encoded characters
    }
  }

  // Case 2: Handle filename (basic or URL-encoded)
  const filenamePart = parts.find(part => part.startsWith('filename='));
  if (filenamePart) {
    // Remove "filename=" and trim quotes (e.g., "filename=report.pdf" → "report.pdf")
    const filename = filenamePart.replace(/^filename=/, '').replace(/["']/g, '');
    return decodeURIComponent(filename); // Decode URL-encoded characters
  }

  return null; // No filename found
};

switch (url.host) {
  case "stage":
    installingWhat = DownloadTypes.Stage;
    break;
  case "avatar":
    installingWhat = DownloadTypes.Avatar;
    break;
  case "beatmap":
    installingWhat = DownloadTypes.Beatmap;
    assetName.value = "Beatmap";
    break;
  case "playlist":
    installingWhat = DownloadTypes.Playlist;
    break;
}

async function makeDownload(url: string, filepath: string) {
  await download(url, filepath, (pr) => {
    downloadProgress.value = Math.round(Math.min((pr.progressTotal / pr.total) * 100, 100));
  });
}


async function doDownload() {
  const synthFolder = await config.get("synth_folder");
  if (typeof synthFolder === "string") {
    if (targetFolder.has(installingWhat)) {
      const downloadFolder = await path.join(synthFolder, targetFolder.get(installingWhat)!);
      const params = url.pathname.split("/").filter(val => val);

      switch (installingWhat) {
        case DownloadTypes.Stage: {
          const id = params[0];
          let targetFile = params[1] ?? null;
          if (id) {
            const result = await fetch(`https://synthriderz.com/api/models/stages/${id}?join=files&join=files.file&fields=name&fields=cover_version`);
            const data = await result.json() as Stage;

            assetName.value = data.name;
            assetAuthor.value = data.user.username;
            cover.value = `https://synthriderz.com/api/models/stages/${id}/cover?v=${data.cover_version}`;

            let filename = null;
            if (!targetFile) {
              const target = data.latest_files.filter((file) => file.platform === "pc").at(-1)!;
              targetFile = target.file.id.toString();
              filename = target.file.filename;
            } else {
              const headData = await fetch(`https://synthriderz.com/api/models/stages/${id}/download?file_id=${targetFile}`, {method: "HEAD"});
              filename = extractFilename(headData.headers.get("content-disposition")!);
            }

            if (filename) {
              const savePath = await path.join(downloadFolder, filename);
              await makeDownload(`https://synthriderz.com/api/models/stages/${id}/download?file_id=${targetFile}`, savePath);
            }
          }
          break;
        }
        case DownloadTypes.Avatar: {
          const id = params[0];
          if (id) {
            const result = await fetch(`https://synthriderz.com/api/models/avatars/${id}`);
            const data = await result.json() as Avatar;
            const filename = data.filename_original;

            assetName.value = data.name;
            assetAuthor.value = data.user.username;
            cover.value = `https://synthriderz.com/api/models/avatars/${id}/cover?v=${data.cover_version}`;

            const savePath = await path.join(downloadFolder, filename);
            await makeDownload(`https://synthriderz.com/api/models/avatars/${id}/download`, savePath);
          }
          break;
        }
        case DownloadTypes.Beatmap: {
          const id = params[0];
          if (id) {
            const headData = await fetch(`https://synthriderz.com/api/beatmaps/hash/download/${id}`, {method: "HEAD"});
            const filename = extractFilename(headData.headers.get("content-disposition")!);

            if (!filename) {
              break;
            }

            assetName.value = filename;
            const savePath = await path.join(downloadFolder, filename);
            await makeDownload(`https://synthriderz.com/api/beatmaps/hash/download/${id}`, savePath);
          }
          break;
        }
        case DownloadTypes.Playlist: {
          const id = params[0];
          if (id) {
            const result = await fetch(`https://synthriderz.com/api/playlists/${id}`);
            const data = await result.json() as Playlist;

            assetName.value = data.name;
            assetAuthor.value = data.user.username;
            cover.value = `https://synthriderz.com/api/playlists/${id}/cover?v=${data.cover_version}`;

            const savePath = await path.join(downloadFolder, data.filename_original);
            await makeDownload(baseSynthAPI + data.download_url, savePath);


          }
          break;
        }

        case DownloadTypes.Undefined:
          break;
      }
    }
  }
}

doDownload().then(() => {
  setTimeout(() => {
    const window = getCurrentWindow();
    window.close();
  }, 2000);
})
</script>

<template>
  <img class="cover"
       :src="cover ?? ''"
       alt="" width="256" height="256">
  <main class="flex flex-col gap-4 justify-end items-center relative py-10">
    <div class="self-center grow place-content-center text-center">
      <p class="text-lg">Installing {{ installingWhat }}...</p>
      <p class="text-2xl font-bold">{{ assetName }}</p>
      <p v-if="assetAuthor">by {{ assetAuthor }}</p>
    </div>
    <div class="radial-progress text-primary" :style="{'--value': downloadProgress}" :aria-valuenow="downloadProgress"
         role="progressbar">{{ downloadProgress }}
    </div>

  </main>
</template>

<style scoped>
main {
  background: linear-gradient(to bottom, transparent 0%, var(--color-base-100) 90%);
  height: 100vh;
}

.cover {
  position: absolute;
  left: 0;
  top: 0;
  width: 100vw;
  height: 100vh;
  object-fit: cover;
  aspect-ratio: 1;
}
</style>