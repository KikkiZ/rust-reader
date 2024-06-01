<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { nextTick, onBeforeUnmount, onMounted, ref, watch } from "vue";

import { useAppStateStore } from "@/store/appStateStore";
import { useSettingStore } from "@/store/settingStore";
import eventBus from "@/utils/eventBus";
import appendPath from "@/utils/commonUtils";
import { ParseType, Parser } from "@/core/contentParser";

const settingStore = useSettingStore();
const appStateStore = useAppStateStore();

const main = ref();
const contentString = ref("");
const contentParser = ref<Parser>();

watch(
    () => settingStore.show_side_bar,
    new_value => {
        show(new_value);
    },
);

watch(
    () => appStateStore.current_chapter,
    async new_value => {
        const result: string = await invoke("jump_to_chapter", { chapter: new_value });
        const { content, success, msg } = JSON.parse(result);

        if (success) {
            contentString.value = content;
        } else {
            eventBus.emit("notices", JSON.parse(msg));
        }
    }
);

watch(
    () => contentString.value,
    () => {
        nextTick().then(() => {
            contentParser.value?.contentParse();

            // 滚动到顶部
            window.scrollTo({
                top: 0,
                behavior: 'smooth',
            });
        });
    },
    { deep: true },
);

function show(flag: boolean) {
    if (flag) {
        main.value.style.marginLeft = 250 + "px";
    } else {
        main.value.style.marginLeft = 0;
    }
}

async function prev_page() {
    const result: string = await invoke("prev_page");
    const { content, success, msg } = JSON.parse(result);

    if (success) {
        contentString.value = content;
        appStateStore.current_chapter -= 1;
    } else {
        eventBus.emit("notices", JSON.parse(msg));
    }
}

async function next_page() {
    const result: string = await invoke("next_page");
    const { content, success, msg } = JSON.parse(result);

    if (success) {
        contentString.value = content;
        appStateStore.current_chapter += 1;
    } else {
        eventBus.emit("notices", JSON.parse(msg));
    }
}

async function open_book(id: string) {
    const result: string = await invoke("open_book", { id: id });
    const { content, success, msg } = JSON.parse(result);

    if (success) {
        contentString.value = content;
        appStateStore.current_chapter = 0;
    } else {
        eventBus.emit("notices", JSON.parse(msg));
    }
}

onMounted(async () => {
    // TODO: 调整接口用法
    const path: string = await invoke("get_resource_path");
    const resource_path = appendPath(path, appStateStore.current_book_id);

    show(settingStore.show_side_bar);
    open_book(appStateStore.current_book_id);

    contentParser.value = new Parser(resource_path, ParseType.Optimize);
});

onBeforeUnmount(() => {
    contentParser.value?.release();
});
</script>

<template>
    <div class="main" ref="main">
        <div id="content" v-html="contentString"></div>

        <div class="row" v-show="contentString">
            <button class="ml-8" @click="prev_page()">
                prev
                <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24">
                    <path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round"
                        stroke-width="2" d="m15 6l-6 6l6 6" />
                </svg>
            </button>
            <button class="ml-8" @click="next_page()">
                next
                <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24">
                    <path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round"
                        stroke-width="2" d="m9 6l6 6l-6 6" />
                </svg>
            </button>
        </div>
    </div>

</template>

<style>
.main {
    padding: 64px 16px;
}
</style>
