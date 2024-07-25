import { defineStore } from "pinia";
import { reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import pinia from ".";

export const useConfigStore = defineStore("config", {
    state: () => {
        const database = ref("");
        const log = ref("");
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

        return { database, log, book, theme, setting };
    },

    actions: {
        // 初始化 store
        initStore(data: {
            database: string;
            log: string;
            book: object;
            theme: object;
            setting: object;
        }) {
            const { database, log, book, theme, setting } = data;

            this.database = database;
            this.log = log;
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
                database: store.database,
                log: store.log,
                book: store.book,
                theme: store.theme,
                setting: store.setting,
            });
            invoke("update_config", { config: newData });
        }
    });
});
