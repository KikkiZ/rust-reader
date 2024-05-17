import { defineStore } from "pinia";
import { ref } from "vue";

export const useAppStateStore = defineStore("appState", () => {
    const current_book_id = ref("");
    const current_chapter = ref(0);

    return { current_book_id, current_chapter };
});