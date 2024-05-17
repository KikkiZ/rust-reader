<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref, watch } from "vue";
import { appWindow } from "@tauri-apps/api/window";

import { useSettingStore } from "@/store/settingStore";

const settingStore = useSettingStore();

watch(
    () => settingStore.show_side_bar,
    new_value => side_bar(new_value),
);

const titlebar = ref();

function side_bar(flag: boolean) {
    settingStore.show_side_bar = flag;
    if (settingStore.show_side_bar) {
        titlebar.value.style.left = 250 + 'px';
    } else {
        titlebar.value.style.left = 0;
    }
    // eventBus.emit("show_side_bar", show_side_bar.value);
}

onMounted(() => {
    side_bar(settingStore.show_side_bar);

    document.getElementById("titlebar-side")!.addEventListener('click', () => settingStore.show_side_bar = !settingStore.show_side_bar);
    document.getElementById("titlebar-minimize")!.addEventListener('click', () => appWindow.minimize());
    document.getElementById("titlebar-maximize")!.addEventListener('click', () => appWindow.toggleMaximize());
    document.getElementById("titlebar-close")!.addEventListener('click', () => appWindow.close());
});

onBeforeUnmount(() => {
    // eventBus.off("show_side_bar");
});
</script>

<template>
    <div data-tauri-drag-region class="titlebar" ref="titlebar">
        <div class="titlebar-button titlebar-side" id="titlebar-side">
            <svg v-show="!settingStore.show_side_bar" xmlns="http://www.w3.org/2000/svg" width="1em" height="1em"
                viewBox="0 0 24 24">
                <g fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2">
                    <path d="M4 6a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2zm5-2v16" />
                    <path d="m14 10l2 2l-2 2" />
                </g>
            </svg>
            <svg v-show="settingStore.show_side_bar" xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24">
                <g fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2">
                    <path d="M4 6a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2zm5-2v16" />
                    <path d="m15 10l-2 2l2 2" />
                </g>
            </svg>
        </div>
        <div>
            <div class="titlebar-button titlebar-minimize" id="titlebar-minimize">
                <img src="https://api.iconify.design/mdi:window-minimize.svg" alt="minimize" />
            </div>
            <div class="titlebar-button titlebar-maximize" id="titlebar-maximize">
                <img src="https://api.iconify.design/mdi:window-maximize.svg" alt="maximize" />
            </div>
            <div class="titlebar-button titlebar-close" id="titlebar-close">
                <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
            </div>
        </div>

    </div>
</template>

<style scoped>
.titlebar {
    height: 30px;
    background: transparent;
    user-select: none;
    display: flex;
    justify-content: space-between;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    padding: 0 3px 0 1.5px;
    /* background-color: green; */
    z-index: 9;
}

.titlebar-button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 26px;
    height: 26px;
    margin: 3px 1.5px;
    border-radius: 6px;
    background-color: rgba(0, 0, 0, 0.05);
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.1);
    -webkit-backdrop-filter: blur(1px);
    backdrop-filter: blur(1px);
}

.titlebar-side>svg {
    height: 20px;
    width: 20px;
}

.titlebar-side {
    background-color: transparent;
    box-shadow: none;
}

.titlebar-side:hover {
    background-color: rgba(0, 0, 0, 0.15);
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.1);
}

.titlebar-minimize:hover {
    background-color: #fdbb40aa;
}

.titlebar-maximize:hover {
    background-color: #35cd4baa;
}

.titlebar-close:hover {
    background-color: #fc625daa;
}
</style>
