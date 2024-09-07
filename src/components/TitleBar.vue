<script setup lang="ts">
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

import router from "@/router";
import { useConfigStore } from "@/store/configStore";

const configStore = useConfigStore();

const appWindow = getCurrentWebviewWindow()
</script>

<template>
    <div data-tauri-drag-region class="titlebar" id="title">
        <div class="button-group">
            <div class="titlebar-button titlebar-side" @click="router.go(-1)">
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
                        stroke-width="2"
                        d="M5 12h14M5 12l6 6m-6-6l6-6" />
                </svg>
            </div>
            <div class="titlebar-button titlebar-side" @click="router.go(1)">
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
                        stroke-width="2"
                        d="M5 12h14m-6 6l6-6m-6-6l6 6" />
                </svg>
            </div>
            <div class="titlebar-button titlebar-side" @click="router.go(0)">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="1em"
                    height="1em"
                    viewBox="0 0 24 24"
                    transform="scale(1, -1)">
                    <path
                        fill="none"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M19.95 11a8 8 0 1 0-.5 4m.5 5v-5h-5" />
                </svg>
            </div>
            <div
                class="titlebar-button titlebar-side"
                @click="
                    configStore.setting.sidebar = !configStore.setting.sidebar
                ">
                <svg
                    v-show="!configStore.setting.sidebar"
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
                    v-show="configStore.setting.sidebar"
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
        </div>

        <div class="button-group group-right">
            <div
                class="titlebar-button titlebar-minimize"
                @click="appWindow.minimize()">
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
                @click="appWindow.toggleMaximize()">
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
            <div
                class="titlebar-button titlebar-close"
                @click="appWindow.close()">
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
    height: 28px;
    background: transparent;
    user-select: none;
    display: flex;
    justify-content: space-between;

    padding: 2px;
    margin-bottom: calc(var(--gap) * -1);
    z-index: 9;
}

.titlebar-button {
    display: inline-flex;
    justify-content: center;
    align-items: center;

    width: 28px;
    height: 28px;
    border-radius: 6px;
    -webkit-backdrop-filter: blur(1px);
    backdrop-filter: blur(1px);
}

.button-group {
    display: flex;
    gap: 8px;
}

.group-right {
    margin-right: 1px;
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
