<script setup lang="ts">
import { PropType, onMounted, ref } from "vue";

import Notification from "@/entity/notification";

const noticeId = ref(Date.now().toString());

function closeNotify(id: string) {
    const current = document.getElementById(id);
    current?.parentNode?.removeChild(current);
}

const props = defineProps({
    notice: { type: Object as PropType<Notification>, required: true },
    duration: { type: Number, default: 5000 },
});

onMounted(() => {
    setTimeout(() => {
        closeNotify(noticeId.value);
    }, props.duration);
});
</script>

<template>
    <div
        v-bind="props.notice"
        :class="notice.type.toString().toLowerCase()"
        class="notification-panel"
        :id="noticeId">
        <div class="notification-close" @click="closeNotify(noticeId)">
            <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
        </div>

        <h3 class="notification-title">{{ notice.title }}</h3>
        <p class="notification-content">{{ notice.msg }}</p>
    </div>
</template>

<style>
.notification-panel {
    width: 250px;
    height: auto;
    border-radius: 6px;
    padding: 8px;
    margin-bottom: 8px;

    -webkit-backdrop-filter: blur(5px);
    backdrop-filter: blur(5px);
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
