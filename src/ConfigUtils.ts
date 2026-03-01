import {Store} from "@tauri-apps/plugin-store";
import {InjectionKey} from "vue";

declare type configStructure = {
    "synth_folder": string,
    "on_existing": "ask" | "skip" | "redownload"
};

export const configKey = Symbol() as InjectionKey<Store>