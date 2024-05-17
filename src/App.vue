<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { onBeforeMount } from "vue";

import TitleBar from "./components/TitleBar.vue";
import Notification from "./components/Notification.vue";
import MainPanel from "./components/MainPanel.vue";
import { useSettingStore } from "./store/settingStore";
// import eventBus from "./eventBus";

onBeforeMount(async () => {
    const result: string = await invoke("get_config");
    const config = JSON.parse(result);
    
    // eventBus.emit("show_side_bar", config.setting.sidebar);
    const setting = useSettingStore();
    setting.show_side_bar = config.setting.sidebar as boolean;
    // setting.current_book_id = "test";
});
</script>

<template>
    <TitleBar />
    <Notification />
    <MainPanel />
</template>
