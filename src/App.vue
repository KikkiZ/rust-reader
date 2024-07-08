<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { onMounted } from "vue";

import TitleBar from "./components/TitleBar.vue";
import MainPanel from "./views/MainPanel.vue";
import { useConfigStore } from "./store/configStore";

const configStore = useConfigStore();

// 初始化 view
onMounted(async () => {
    const result: string = await invoke("get_config");
    const config = JSON.parse(result);
    configStore.initStore(config);
});
</script>

<template>
    <TitleBar />
    <MainPanel />
    <div id="notification-panel"></div>
</template>

<style scoped>
#notification-panel {
    z-index: 3;
    position: fixed;
    bottom: 12px;
    right: 12px;
    display: flex;
    flex-direction: column;
    gap: 4px;
}
</style>
