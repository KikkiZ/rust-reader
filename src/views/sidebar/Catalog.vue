<script setup lang="ts">
import { nextTick, onMounted, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import router from "@/router";
import { useSettingStore } from "@/store/settingStore";
import { useAppStateStore } from "@/store/appStateStore";
import { refreshView } from "@/core/sidebarControl";
import { notify } from "@/core/notifyService";
import ToolBox from "@/components/ToolBox.vue";

const settingStore = useSettingStore();
const appStateStore = useAppStateStore();

const catalogList = ref();

const catalog = ref<Array<String>[]>([]);

watch(
    () => appStateStore.current_book_id,
    () => {
        refreshCatalog();
    },
);

watch(
    () => appStateStore.current_chapter,
    (newValue, oldValue) => {
        console.log(newValue, oldValue);
        const list = catalogList.value.children;
        list[newValue].classList.add("current");
        list[oldValue].classList.remove("current");
    },
);

// 刷新目录
// 只有在 appStateStore.current_book_id 发生改变或首次打开
// 书本时会调用该方法, 因此, 需要给目录第一项添加 current 类
async function refreshCatalog() {
    const result: string = await invoke("get_book_catalog");
    const { catalog: catalog_data, success, msg } = JSON.parse(result);

    if (success) {
        catalog.value = catalog_data;
        nextTick(() => {
            catalogList.value.children[0].classList.add("current");
        });
    } else {
        notify(msg);
    }
}

function back() {
    router.push("/list");
}

onMounted(() => {
    refreshView(settingStore.show_side_bar);
    refreshCatalog();
});
</script>

<template>
    <div class="sidebar" id="side">
        <button @click="back()">back</button>
        <div class="catalog" ref="catalogList">
            <div
                v-for="(item, index) in catalog"
                @click="appStateStore.current_chapter = index"
                class="catalog-item">
                <p>{{ item }}</p>
            </div>
        </div>
        <ToolBox />
    </div>
</template>

<style scoped>
.sidebar {
    box-sizing: border-box;
    overflow-x: hidden;
    overflow-y: scroll;
}

.catalog {
    padding: 0;
    margin: 8px 0;
    display: flex;
    flex-direction: column;
    gap: 4px;
}

.catalog-item {
    border-radius: 8px;
    border: none;
    padding: 8px;
}

.catalog-item:hover {
    background-color: rgba(0, 0, 0, 0.1);
}

.catalog-item > p {
    margin: 0;
    font-size: 20px;
    text-indent: 0;
}

.current {
    background-color: #396cd8;
    color: white;
    border-radius: 8px;
}

.current:hover {
    background-color: #396cd8;
}
</style>
