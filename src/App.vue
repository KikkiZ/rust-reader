<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { onMounted } from "vue";

import TitleBar from "./components/TitleBar.vue";
import Notification from "./components/Notification.vue";
import MainPanel from "./views/MainPanel.vue";
import { useSettingStore } from "./store/settingStore";

// 初始化 view
onMounted(async () => {
    const result: string = await invoke("get_config");
    const config = JSON.parse(result);

    const setting = useSettingStore();
    setting.show_side_bar = config.setting.sidebar as boolean;
});
</script>

<template>
    <TitleBar />
    <Notification />
    <MainPanel />
</template>
