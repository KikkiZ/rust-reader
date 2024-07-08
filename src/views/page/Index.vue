<script setup lang="ts">
import { onMounted, ref } from "vue";

import { useConfigStore } from "@/store/configStore";
import { refreshView } from "@/core/sidebarControl";

const configStore = useConfigStore();

const greetString = ref("");

onMounted(() => {
    refreshView(configStore.setting.sidebar);

    const hour = new Date().getHours();
    if (hour >= 0 && hour < 6) {
        greetString.value = "夜深了！";
    } else if (hour >= 6 && hour < 12) {
        greetString.value = "早上好！";
    } else if (hour >= 12 && hour < 18) {
        greetString.value = "下午好！";
    } else {
        greetString.value = "晚上好！";
    }
});
</script>

<template>
    <div class="main">
        <h1 class="greet">
            {{ greetString }}
        </h1>
    </div>
</template>

<style scoped>
.greet {
    margin: 0;
    margin-left: 2em;
    font-size: 2em;
    line-height: 2em;
    text-align: start;
    color: #353535;
}
</style>
