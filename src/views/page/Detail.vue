<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";

import BookInfo from "@/entity/bookInfo";
import { useAppStateStore } from "@/store/appStateStore";
import { useConfigStore } from "@/store/configStore";
import { notify } from "@/core/notifyService";
import { refreshView } from "@/core/sidebarControl";

const configStore = useConfigStore();
const appStateStore = useAppStateStore();

const detail = ref<BookInfo>();

watch(
    () => appStateStore.current_book_id,
    () => {
        showDetail(appStateStore.current_book_id);
    },
);

async function showDetail(id: string) {
    const result: string = await invoke("book_detail", { id: id });
    const { exist, info, msg } = JSON.parse(result);

    if (exist) {
        console.log(info);
        info.cover_path = convertFileSrc(info.cover_path);
        detail.value = info;
    } else {
        notify(msg);
    }
}

onMounted(() => {
    refreshView(configStore.setting.sidebar);
    showDetail(appStateStore.current_book_id);
});
</script>

<template>
    <div class="detail">
        <div class="cover">
            <img :src="detail?.cover_path" />
        </div>
        <div class="info-panel">
            <h1 class="title">{{ detail?.title }}</h1>
            <p class="creator">{{ detail?.creator }}</p>
            <p class="description" v-if="detail?.description !== ''">
                {{ detail?.description }}
            </p>
        </div>
    </div>
</template>

<style scoped>
.detail {
    display: flex;
    gap: 16px;
}

.detail .cover {
    max-width: 200px;
    min-width: 200px;
}

.detail .cover img {
    width: 100%;
    border-radius: 8px;
    box-shadow: 2px 4px 4px rgba(0, 0, 0, 0.15);
}

.detail .info-panel {
    display: flex;
    flex-direction: column;
}

.detail .info-panel .title {
    margin: 12px 0;
    font-size: 2rem;
    line-height: 40px;
    text-align: start;
}

.detail .info-panel .creator {
    margin: 0;
    font-size: 1.25rem;
    line-height: 32px;
    text-align: start;
}
</style>
