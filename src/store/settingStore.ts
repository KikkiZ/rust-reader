import { defineStore } from "pinia";
import { ref } from "vue";

export const useSettingStore = defineStore("setting", () => {
    const show_side_bar = ref(true);

    return { show_side_bar };
});
