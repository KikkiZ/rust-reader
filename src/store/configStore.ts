import { defineStore } from "pinia";
import { reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import pinia from ".";

export const useConfigStore = defineStore("config", {
    state: () => {
        const book = reactive({
            info: ref(""),
            dir: ref(""),
            cover: ref(""),
            resources: ref(""),
        });
        const theme = reactive({
            appearance: ref(""),
        });
        const setting = reactive({
            sidebar: ref(true),
        });

        return { book, theme, setting };
    },

    actions: {
        // 初始化 store
        initStore(data: { book: object; theme: object; setting: object }) {
            const { book, theme, setting } = data;

            this.book = book as {
                info: string;
                dir: string;
                cover: string;
                resources: string;
            };
            this.theme = theme as { appearance: string };
            this.setting = setting as { sidebar: boolean };
        },
    },
});

// 监听 pinia 中 configStore 的变化
pinia.use(({ store }) => {
    store.$subscribe(() => {
        if (store.$id === "config") {
            const newData = JSON.stringify({
                book: store.book,
                theme: store.theme,
                setting: store.setting,
            });
            invoke("update_config", { config: newData });
        }
    });
});
