<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";

import BookInfo from "@/entity/bookInfo";
import Notification from "@/entity/notification";
import router from "@/router";
import { useSettingStore } from "@/store/settingStore";
import { useAppStateStore } from "@/store/appStateStore";
import { refreshView } from "@/core/sidebarControl";
import { notify } from "@/core/notifyService";

const settingStore = useSettingStore();
const appStateStore = useAppStateStore();

const form = ref();
const list = ref();
const keyWord = ref("");
const items = ref<BookInfo[]>([]);
const clickTimeout = ref(-1);
const selectItem = ref("");

watch(selectItem, (newValue, oldValue) => {
    const newItem = document.getElementById(newValue);
    const oldItem = document.getElementById(oldValue);

    newItem?.classList.add("selected");
    oldItem?.classList.remove("selected");
});

async function updateBook() {
    const selected = (await open({
        multiple: true,
        filters: [
            {
                name: "e-book",
                extensions: ["epub"],
            },
        ],
    })) as string[];

    if (selected && selected.length > 0) {
        const result: string = await invoke("update_new_book", {
            paths: selected,
        });
        const messages: Notification[] = JSON.parse(result);

        for (const index in messages) {
            setTimeout(() => {
                notify(messages[index]);
            }, 300 * parseInt(index));
        }

        getBookList();
    }
}

async function searchBook() {
    if (keyWord.value !== "") {
        await invoke("search_book", { key: keyWord.value });
    }
}

async function getBookList() {
    const msg: string = await invoke("book_list");
    items.value = JSON.parse(msg);
    items.value.forEach((item) => {
        item.cover_path = convertFileSrc(item.cover_path);
        item.date = item.date.toString().slice(0, 4);
    });
}

function openDetail(id: string) {
    if (clickTimeout.value > 0) {
        clearTimeout(clickTimeout.value);
        clickTimeout.value = -1;
    }

    clickTimeout.value = window.setTimeout(() => {
        invoke("book_detail", { id: id });
        appStateStore.current_book_id = id;
        selectItem.value = id;

        try {
            router.push("/detail");
        } catch (error) {
            console.log(error);
        }

        clickTimeout.value = -1;
    }, 300);
}

function openBook(id: string) {
    if (clickTimeout.value > 0) {
        clearTimeout(clickTimeout.value);
        clickTimeout.value = -1;
    }

    invoke("open_book", { id: id });
    appStateStore.current_book_id = id;

    try {
        router.push("/read");
    } catch (error) {
        console.log(error);
    }
}

onMounted(() => {
    refreshView(settingStore.show_side_bar);
    getBookList();
});
</script>

<template>
    <div class="sidebar" id="side">
        <form class="search-form" @submit.prevent="searchBook" ref="form">
            <input type="text" v-model="keyWord" placeholder="Search book..." />
            <button type="submit" class="search-button">
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
                        d="M3 10a7 7 0 1 0 14 0a7 7 0 1 0-14 0m18 11l-6-6" />
                </svg>
            </button>
        </form>

        <div class="book-list" ref="list">
            <div
                class="book-list-item"
                v-for="item in items"
                :id="item.id"
                @click="openDetail(item.id)"
                @dblclick="openBook(item.id)">
                <div class="book-cover">
                    <img :src="item.cover_path" />
                </div>

                <div class="book-info-panel">
                    <h4 class="book-title">{{ item.title }}</h4>
                    <p class="book-info">
                        {{ item.creator }} / {{ item.date }}
                    </p>
                </div>
            </div>
        </div>

        <button @click="updateBook()" style="margin-top: 8px">
            update
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
                    d="M12 19H5a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h4l3 3h7a2 2 0 0 1 2 2v3.5M19 22v-6m3 3l-3-3l-3 3" />
            </svg>
        </button>
    </div>
</template>

<style scoped>
.sidebar {
    box-sizing: border-box;
    overflow-x: hidden;
    overflow-y: scroll;
}

.search-form {
    width: 244px;

    top: 0;
    position: sticky;
}

.search-form > input {
    width: 168px;
    height: 40px;
    padding: 0 15px;
    border: 1px solid #e8e8e8;
}

.search-form > button {
    width: 40px;
    height: 40px;
    padding: 0;
    margin-left: 4px;
    border: 1px solid #e8e8e8;
}

.search-form > input,
.search-form > button {
    box-shadow: none;
}

.search-form > input:hover,
.search-form > button:hover {
    border-color: #396cd8;
}

.book-list {
    margin: 0px;
    margin-top: 8px;
    display: flex;
    gap: 4px;
    flex-direction: column;
    overflow: scroll;
}

.book-list-item {
    border-radius: 8px;
    border: none;
    margin-top: 0;
    padding: 8px;

    display: flex;
}

.book-list-item:hover {
    background-color: rgba(0, 0, 0, 0.1);
}

.book-list-item > .book-cover {
    width: 60px;
}

.book-list-item > .book-cover > img {
    width: 100%;
    height: auto;
    object-fit: cover;

    border-radius: 6px;
    box-shadow: 2px 4px 4px rgba(0, 0, 0, 0.15);
}

.book-list-item > .book-info-panel {
    margin-left: 8px;
    width: 80%;
}

.book-info-panel > .book-title {
    margin: 0;
    margin-top: 4px;
    font-size: 20px;
    line-height: 20px;
    color: #2d2d2d;
}

.book-info-panel > .book-info {
    margin: 0;
    margin-top: 8px;
    color: #525252;
    font-size: 16px;
    line-height: 16px;
}

.selected {
    background-color: rgba(0, 0, 0, 0.1);
}
</style>
