<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";

import BookInfo from "@/entity/bookInfo";
import Notification from "@/entity/notification";
import router from "@/router";
import eventBus from "@/utils/eventBus";
import { useSettingStore } from "@/store/settingStore";
import { useAppStateStore } from "@/store/appStateStore";

const settingStore = useSettingStore();
const appStateStore = useAppStateStore();

watch(
	() => settingStore.show_side_bar,
	new_value => {
		show(new_value);
	},
);

const sidebar = ref();
async function show(flag: boolean) {
	if (!flag) {
		sidebar.value.style.display = "none";
	} else {
		sidebar.value.style.display = "block";
	}
}

const form = ref();
const list = ref();

async function update_book() {
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
		const result: string = await invoke("update_new_book", { paths: selected });
		const messages: Notification[] = JSON.parse(result);

		for (const index in messages) {
			setTimeout(() => {
				eventBus.emit("notices", messages[index]);
			}, 300 * parseInt(index));
		}

		get_book_list();
	}
}

const key_word = ref("");
async function search_book() {
	if (key_word.value !== "") {
		await invoke("search_book", { key: key_word.value });
	}
}

const items = ref<BookInfo[]>([]);
async function get_book_list() {
	const msg: string = await invoke("book_list");
	items.value = JSON.parse(msg);
	items.value.forEach((item) => {
		item.cover_path = convertFileSrc(item.cover_path);
		item.date = item.date.toString().slice(0, 4);
	});
}

function open_book(id: string) {
	invoke("open_book", { id: id });
	appStateStore.current_book_id = id;

	try {
		router.push("/read");
	} catch (error) {
		console.log(error);
	}
}

onMounted(() => {
	show(settingStore.show_side_bar);
	get_book_list();
});
</script>

<template>
	<div class="sidebar" ref="sidebar">
		<form class="search-form" @submit.prevent="search_book" ref="form">
			<input type="text" v-model="key_word" placeholder="Search book..." />
			<button type="submit" class="search-button">
				<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24">
					<path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round"
						stroke-width="2" d="M3 10a7 7 0 1 0 14 0a7 7 0 1 0-14 0m18 11l-6-6" />
				</svg>
			</button>
			<button class="ml-8" @click="update_book()">
				update
				<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24">
					<path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round"
						stroke-width="2"
						d="M12 19H5a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h4l3 3h7a2 2 0 0 1 2 2v3.5M19 22v-6m3 3l-3-3l-3 3" />
				</svg>
			</button>
		</form>

		<ul class="book-list" ref="list">
			<li v-for="item in items" class="book-list-item" :id="item.id" @click="open_book(item.id)">
				<div class="book-cover">
					<img :src="item.cover_path" />
				</div>

				<div class="book-info-panel">
					<h4 class="book-title">{{ item.title }}</h4>
					<p class="book-info">{{ item.creator }} / {{ item.date }}</p>
				</div>
			</li>
		</ul>
	</div>
</template>

<style scoped>
.search-form {
	width: 234px;
	margin: 8px 8px 0 8px;

	top: 8px;
	position: sticky;
	background-color: white;
}

.search-form>input {
	width: 158px;
	height: 40px;
	padding: 0 15px;
	border: 1px solid #e8e8e8;
}

.search-form>button {
	width: 40px;
	height: 40px;
	padding: 0;
	margin-left: 4px;
	border: 1px solid #e8e8e8;
}

.search-form>input,
.search-form>button {
	/* box-shadow: 0 2px 4px rgba(0, 0, 0, 0.15); */
	box-shadow: none;
}

.search-form>input:hover,
.search-form>button:hover {
	border-color: #396cd8;
}

.book-list {
	list-style-type: none;
	/* height: 500px; */
	/* height: auto; */
	margin: 0px;
	padding: 8px;
	overflow: scroll;
	/* display: flex; */
}

.book-list-item {
	border-radius: 0px;
	border: none;
	border-top: 1px solid #e8e8e8;
	margin-top: 0;
	padding: 8px;

	display: flex;
	/* transition: background-color 0.3s ease-in-out, border-radius 0.2s ease-in-out; */
}

.book-list-item:hover {
	background-color: #e8e8e8;
	border-radius: 8px;
	border-top: 1px solid transparent;
}

.book-list-item:hover+.book-list-item {
	border-top: 1px solid transparent;
}

.book-list-item:first-child {
	border: none;
}

.book-list-item>.book-cover {
	width: 60px;
}

.book-list-item>.book-info-panel {
	margin-left: 8px;
}

.book-info-panel>.book-title {
	margin: 0;
	margin-top: 4px;
	/* line-height: 30px; */
	font-size: 20px;
	line-height: 20px;
	color: #363636;
}

.book-info-panel>.book-info {
	margin: 0;
	margin-top: 8px;
	color: #808080;
	font-size: 16px;
	line-height: 16px;
}
</style>
