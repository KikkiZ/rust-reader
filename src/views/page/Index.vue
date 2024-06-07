<script setup lang="ts">
import { onMounted } from "vue";

import { useSettingStore } from "@/store/settingStore";
import { refreshView } from "@/core/sidebarControl";

const settingStore = useSettingStore();

onMounted(() => {
    refreshView(settingStore.show_side_bar);
});
</script>

<template>
    <div class="main" id="main">
        hello
    </div>
</template>

<style></style>
