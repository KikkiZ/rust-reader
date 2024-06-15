<script setup lang="ts">
import { onBeforeUnmount, ref, watch } from "vue";

import Notification from "@/entity/notification";
import eventBus from "@/utils/eventBus";

const notices = ref<Notification[]>([]);

eventBus.on("notices", (notice) => {
    notices.value.push(notice as Notification);
});

function nitifationClose(index: number) {
    notices.value.splice(index, 1);
}

watch(
    notices,
    async (new_notice) => {
        if (new_notice.length > 0) {
            setTimeout(() => {
                notices.value.shift();
            }, 5000);
        }
    },
    { deep: true },
);

onBeforeUnmount(() => {
    eventBus.off("notices");
});
</script>

<template>
    <div class="notification-container">
        <div
            v-for="(notice, index) in notices"
            :key="index"
            :class="notice.type.toString().toLowerCase()"
            class="notification-panel">
            <div class="notification-close" @click="nitifationClose(index)">
                <img
                    src="https://api.iconify.design/mdi:close.svg"
                    alt="close" />
            </div>

            <h3 class="notification-title">{{ notice?.title }}</h3>
            <p class="notification-content">{{ notice?.msg }}</p>
        </div>
    </div>
</template>

<style>
.notification-container {
    display: flex;
    flex-direction: column;
    overflow-y: auto;

    position: fixed;
    bottom: 16px;
    right: 16px;
}

.notification-panel {
    width: 250px;
    height: auto;
    border-radius: 8px;
    padding: 8px;
    margin-bottom: 8px;

    -webkit-backdrop-filter: blur(1px);
    backdrop-filter: blur(1px);
}

.notification-panel:last-child {
    margin-bottom: 0;
}

.info {
    background-color: #35cd4b80;
}

.warn {
    background-color: #fdbb4080;
}

.err {
    background-color: #fc625d80;
}

.notification-panel > .notification-close {
    position: fixed;
    top: 8px;
    right: 8px;
    width: 16px;
    height: 16px;
    border-radius: 50%;
}

.notification-panel > .notification-close:hover {
    background-color: rgba(0, 0, 0, 0.15);
}

.notification-panel > .notification-title {
    margin: 0;
}

.notification-panel > .notification-content {
    margin: 0;
}
</style>
