<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import router from "@/router";
import { useSettingStore } from "@/store/settingStore";
import { useAppStateStore } from "@/store/appStateStore";
import eventBus from "@/utils/eventBus";

const settingStore = useSettingStore();
const appStateStore = useAppStateStore();

const catalog_list = ref();

watch(
    () => settingStore.show_side_bar,
    new_value => {
        show(new_value);
    },
);

watch(
    () => appStateStore.current_book_id,
    () => {
        refresh_catalog();
    },
)

watch(
    () => appStateStore.current_chapter,
    (new_value, old_value) => {
        console.log(new_value, old_value)
        const list = catalog_list.value.children;
        list[new_value].classList.add("current")
        list[old_value].classList.remove("current")
    },
)

const sidebar = ref();
async function show(flag: boolean) {
    if (!flag) {
        sidebar.value.style.display = "none";
    } else {
        sidebar.value.style.display = "block";
    }
}

const catalog = ref<Array<String>[]>([]);
async function refresh_catalog() {
    const result: string = await invoke("get_book_catalog");
    const { catalog: catalog_data, success, msg } = JSON.parse(result);

    if (success) {
        catalog.value = catalog_data;
    } else {
        eventBus.emit("notices", msg)
    }
}

function back() {
    router.push("/list");
}

onMounted(() => {
    show(settingStore.show_side_bar);
    refresh_catalog();
})
</script>

<template>
    <div class="sidebar" ref="sidebar">
        <button @click="back()">back</button>
        <ul class="catalog" ref="catalog_list">
            <li v-for="(item, index) in catalog" @click="appStateStore.current_chapter = index" class="catalog-item">
                <p>{{ item }}</p>
            </li>
        </ul>
    </div>
</template>

<style scoped>
.sidebar {
    box-sizing: border-box;
    padding: 8px;
    overflow: scroll;
}

.catalog {
    list-style: none;
    padding: 0;
}

.catalog-item {
    border-radius: 0px;
    border: none;
    border-top: 1px solid #e8e8e8;
    padding: 8px;
}

.catalog-item:first-child {
    border-top: none;
}

.catalog-item:hover {
    background-color: #e8e8e8;
    border-radius: 8px;
    border-top: 1px solid transparent;
}

.catalog-item:hover+.catalog-item {
    border-top: 1px solid transparent;
}

.catalog-item>p {
    margin: 0;
    font-size: 20px;
    text-indent: 0;
}

.current {
    background-color: #396cd8;
    border-radius: 8px;
    border-top: 1px solid transparent;
}

.current+.catalog-item {
    border-top: 1px solid transparent;
}
</style>
