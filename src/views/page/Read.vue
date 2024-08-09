<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { nextTick, onBeforeUnmount, onMounted, ref, watch } from "vue";

import { appendPath } from "@/utils/commonUtils";
import { useAppStateStore } from "@/store/appStateStore";
import { useConfigStore } from "@/store/configStore";
import { Parser } from "@/core/contentParser";
import { refreshView } from "@/core/sidebarControl";
import { notify } from "@/core/notifyService";
import { addBookMark, refreshBookMark, setContent } from "@/core/bookmarkService";

const configStore = useConfigStore();
const appStateStore = useAppStateStore();

const main = ref();
const contentString = ref("");
const contentParser = ref<Parser>();

watch(
    () => appStateStore.current_chapter,
    async (newValue) => {
        const result: string = await invoke("jump_to_chapter", {
            chapter: newValue,
        });
        const { content, success, msg } = JSON.parse(result);

        if (success) {
            contentString.value = content;
        } else {
            notify(msg);
        }
    },
);

watch(
    () => contentString.value,
    () => {
        nextTick().then(() => {
            contentParser.value?.contentParse();
            setContent(
                document
                    .getElementById("content")!
                    .cloneNode(true) as HTMLElement,
            );

            // 滚动到顶部
            const start = main.value.scrollTop;
            const change = -start;
            const startTime = performance.now();

            function animateScroll(currentTime: number) {
                const elapsed = currentTime - startTime;
                const progress = Math.min(elapsed / 300, 1); // 动画执行的进度
                main.value.scrollTop = start + change * progress;

                if (progress < 1) {
                    requestAnimationFrame(animateScroll);
                }
            }

            // 该函数由浏览器提供, 用于创建动画帧
            // 该函数接收一个回调函数作为参数, 在回调函数中
            // 更新元素的变化, 并可以根据时间戳来判断动画是否完成
            requestAnimationFrame(animateScroll);

            refreshBookMark();
        });
    },
    { deep: true },
);

async function prevPage() {
    const result: string = await invoke("prev_page");
    const { content, success, msg } = JSON.parse(result);

    if (success) {
        contentString.value = content;
        appStateStore.current_chapter -= 1;
    } else {
        notify(msg);
    }
}

async function nextPage() {
    const result: string = await invoke("next_page");
    const { content, success, msg } = JSON.parse(result);

    if (success) {
        contentString.value = content;
        appStateStore.current_chapter += 1;
    } else {
        notify(msg);
    }
}

async function openBook(id: string) {
    const result: string = await invoke("open_book", { id: id });
    const { content, success, msg } = JSON.parse(result);

    if (success) {
        contentString.value = content;
        appStateStore.current_chapter = 0;
    } else {
        notify(msg);
    }
}

onMounted(async () => {
    refreshView(configStore.setting.sidebar);

    // TODO: 调整接口用法
    const path: string = await invoke("get_resource_path");
    const resourcePath = appendPath(path, appStateStore.current_book_id);

    openBook(appStateStore.current_book_id);

    contentParser.value = new Parser(resourcePath);

    document
        .getElementById("content")!
        .addEventListener("dblclick", addBookMark);
});

onBeforeUnmount(() => {
    document
        .getElementById("content")!
        .removeEventListener("dblclick", addBookMark);
    contentParser.value?.release();
});
</script>

<template>
    <div class="main" ref="main">
        <div id="content" v-html="contentString"></div>

        <div class="row" v-show="contentString">
            <button class="ml-8" @click="prevPage()">
                prev
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
                        d="m15 6l-6 6l6 6" />
                </svg>
            </button>
            <button class="ml-8" @click="nextPage()">
                next
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
                        d="m9 6l6 6l-6 6" />
                </svg>
            </button>
        </div>
    </div>
</template>

<style scoped></style>
