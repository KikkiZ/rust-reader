import { defineStore } from "pinia";
import { ref } from "vue";

export const useAppStateStore = defineStore(
    "appState",
    () => {
        const current_book_id = ref("");
        const current_chapter = ref(0);
        const sidebar_width = ref(250);

        return { current_book_id, current_chapter, sidebar_width };
    },
    {
        persist: {
            key: "appState",
            storage: localStorage,
        },
    },
);
