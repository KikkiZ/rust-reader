<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

import BookInfo from "@/entity/bookInfo";
import { notify } from "@/core/notifyService";

const msg = ref("");
const content = ref("");
const items = ref<BookInfo[]>([]);

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
        invoke("update_new_book", { paths: selected });
    }
}

async function getBookList() {
    msg.value = await invoke("book_list");
    items.value = JSON.parse(msg.value);
    items.value.forEach((item) => {
        item.cover_path = convertFileSrc(item.cover_path);
    });
}

async function itemClick(id: string) {
    const result: string = await invoke("open_book", { id: id });
    const { content, success, msg } = JSON.parse(result);

    if (success) {
        content.value = content;
    } else {
        notify(msg);
    }
}

onMounted(() => getBookList());
</script>

<template>
    <div class="row">
        <button class="ml-8" @click="updateBook()">
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
    <ul class="book-list">
        <li
            class="book-list-item"
            :id="item.id"
            @click="itemClick(item.id)"
            v-for="item in items">
            <div class="book-cover">
                <img :src="item.cover_path" />
            </div>

            <div class="book-info-panel">
                <h2 class="item-title">{{ item.title }}</h2>
                <p>
                    {{ item }}
                </p>
            </div>
        </li>
    </ul>

    <div v-html="content"></div>
</template>

<style scoped>
.book-list {
    list-style-type: none;
    margin: 0px;
    padding: 0px;
}

.book-list-item {
    background-color: #ffffff;
    border-radius: 8px;
    border: 1px solid #e8e8e8;
    margin-top: 8px;
    padding: 16px;

    display: flex;
}

.book-list-item:hover {
    background-color: #e8e8e8;
}

.book-list-item > .book-cover {
    width: 200px;
}

.book-list-item > .book-info-panel {
    margin-left: 12px;
    width: 80%;
}
</style>
