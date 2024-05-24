<script setup lang="ts">
import { onMounted, ref, watch } from "vue";

import { useSettingStore } from "@/store/settingStore";

const settingStore = useSettingStore();

watch(
    () => settingStore.show_side_bar,
    flag => show(flag),
);

let main = ref();
function show(flag: boolean) {
    if (flag) {
        main.value.style.marginLeft = 250 + "px";
    } else {
        main.value.style.marginLeft = 0;
    }
}

onMounted(() => {
    show(settingStore.show_side_bar);
});

</script>

<template>
    <div class="main" ref="main">
        hello
    </div>
</template>

<style></style>
