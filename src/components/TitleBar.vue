<script setup lang="ts">
import { onMounted } from "vue";
import { appWindow } from "@tauri-apps/api/window";

import { useSettingStore } from "@/store/settingStore";

const settingStore = useSettingStore();

onMounted(() => {
    document
        .getElementById("titlebar-side")!
        .addEventListener(
            "click",
            () => (settingStore.show_side_bar = !settingStore.show_side_bar),
        );
    document
        .getElementById("titlebar-minimize")!
        .addEventListener("click", () => appWindow.minimize());
    document
        .getElementById("titlebar-maximize")!
        .addEventListener("click", () => appWindow.toggleMaximize());
    document
        .getElementById("titlebar-close")!
        .addEventListener("click", () => appWindow.close());
});
</script>

<template>
    <div data-tauri-drag-region class="titlebar" id="title">
        <div class="titlebar-button titlebar-side" id="titlebar-side">
            <svg
                v-show="!settingStore.show_side_bar"
                xmlns="http://www.w3.org/2000/svg"
                width="1em"
                height="1em"
                viewBox="0 0 24 24">
                <g
                    fill="none"
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2">
                    <path
                        d="M4 6a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2zm5-2v16" />
                    <path d="m14 10l2 2l-2 2" />
                </g>
            </svg>
            <svg
                v-show="settingStore.show_side_bar"
                xmlns="http://www.w3.org/2000/svg"
                width="1em"
                height="1em"
                viewBox="0 0 24 24">
                <g
                    fill="none"
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2">
                    <path
                        d="M4 6a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2zm5-2v16" />
                    <path d="m15 10l-2 2l2 2" />
                </g>
            </svg>
        </div>
        <div class="button-group">
            <div
                class="titlebar-button titlebar-minimize"
                id="titlebar-minimize">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="1em"
                    height="1em"
                    viewBox="0 0 24 24">
                    <path
                        fill="none"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2.5"
                        d="M5 13h14" />
                </svg>
            </div>
            <div
                class="titlebar-button titlebar-maximize"
                id="titlebar-maximize">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="1em"
                    height="1em"
                    viewBox="0 0 24 24">
                    <path
                        fill="none"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2.5"
                        d="M4 6a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2z" />
                </svg>
            </div>
            <div class="titlebar-button titlebar-close" id="titlebar-close">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="1em"
                    height="1em"
                    viewBox="0 0 24 24">
                    <path
                        fill="none"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2.5"
                        d="M18 6L6 18M6 6l12 12" />
                </svg>
            </div>
        </div>
    </div>
</template>

<style scoped>
.titlebar {
    height: 24px;
    background: transparent;
    user-select: none;
    display: flex;
    justify-content: space-between;

    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    padding: 7px 7px 0 7px;
    z-index: 9;
}

.titlebar-button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 24px;
    height: 24px;
    /* margin: 1px 1.5px; */
    border-radius: 6px;
    /* background-color: rgba(0, 0, 0, 0.05); */
    /* box-shadow: 0 2px 2px rgba(0, 0, 0, 0.1); */
    -webkit-backdrop-filter: blur(1px);
    backdrop-filter: blur(1px);
}

.button-group {
    display: flex;
    /* gap: 4px; */
}

.titlebar-maximize {
    margin-left: 3px;
}

.titlebar-button svg {
    height: 22px;
    width: 22px;
}

.titlebar-maximize svg {
    height: 16px;
    width: 16px;
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
