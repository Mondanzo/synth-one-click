import {createApp} from "vue";
import {load} from "@tauri-apps/plugin-store";
import App from "./App.vue";
import {configKey} from "./ConfigUtils.ts";

const app = createApp(App);

(async () => {
    const config = await load("config.json", {defaults: {}, autoSave: 500});
    app.provide(configKey, config);


    app.mount("#app");
})();