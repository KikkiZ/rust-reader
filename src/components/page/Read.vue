<script setup lang="ts">
import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import { nextTick, onBeforeUnmount, onMounted, ref, watch } from "vue";

import { useAppStateStore } from "@/store/appStateStore";
import { useSettingStore } from "@/store/settingStore";
import eventBus from "@/utils/eventBus";
import appendURL from "@/utils/commonUtils";

const settingStore = useSettingStore();
const appStateStore = useAppStateStore();

watch(
    () => settingStore.show_side_bar,
    new_value => {
        show(new_value);
    },
);

watch(
    () => appStateStore.current_chapter,
    async new_value => {
        const result: string = await invoke("jump_to_chapter", { chapter: new_value });
        page_refresh(result);
    }
)

let main = ref();
function show(flag: boolean) {
    if (flag) {
        main.value.style.marginLeft = 250 + "px";
    } else {
        main.value.style.marginLeft = 0;
    }
}

let contentString = ref("");
async function prev_page() {
    const result: string = await invoke("prev_page");
    page_refresh(result);
}

async function next_page() {
    const result: string = await invoke("next_page");
    page_refresh(result);
}

async function open_book(id: string) {
    const result: string = await invoke("open_book", { id: id });
    page_refresh(result);
}

function page_refresh(data: string) {
    const { content, success, msg } = JSON.parse(data);

    if (success) {
        contentString.value = content;
    } else {
        eventBus.emit("notices", JSON.parse(msg));
    }
}

watch(
    () => contentString.value,
    () => {
        nextTick().then(() => {
            let contents = document.getElementById("content")?.children;

            if (contents === undefined) {
                return;
            }

            for (const content of contents) {
                traversal(content);
            }

            // 滚动到顶部
            window.scrollTo({
                top: 0,
                behavior: 'smooth',
            });
        });
    },
    { deep: true },
);

function traversal(node: HTMLElement | ChildNode) {
    let element;
    if (node.nodeType === Node.ELEMENT_NODE) {
        element = node as HTMLElement;
    }

    let children = node.childNodes;
    if (children.length === 0 && element !== undefined) {

        // console.log(element.nodeType, element.nodeName, element.nodeValue)
        if (element.hasAttribute("src")) {
            element = element as HTMLImageElement;

            let target = element.src.substring(7);
            // target = resource_path.value + "\\" + target;
            target = appendURL(resource_path.value, target);

            element.src = convertFileSrc(target);
        } else if (element.hasAttribute("href")) {
            element = element as HTMLAnchorElement;

            let target = element.href.substring(7);
            // target = resource_path.value + "\\" + target;
            target = appendURL(resource_path.value, target);

            element.href = convertFileSrc(target);
        } else if (element.hasAttribute("xlink:href")) {
            element = element as unknown as SVGImageElement;

            let target = element.href.baseVal.substring(7);
            // target = resource_path.value + "\\" + target;
            target = appendURL(resource_path.value, target);

            element.href.baseVal = convertFileSrc(target);
        }

        return;
    }

    for (const current of children) {
        traversal(current);
    }
}

const resource_path = ref("");

const dynamic_css = ref<string[]>([]);
onMounted(async () => {
    resource_path.value = await invoke("get_resource_path");
    resource_path.value = appendURL(resource_path.value, appStateStore.current_book_id);
    
    show(settingStore.show_side_bar);
    open_book(appStateStore.current_book_id);

    // 动态加载css
    const result: string = await invoke("get_css");
    const { success, css } = JSON.parse(result);

    if (success) {
        dynamic_css.value = Object.keys(css);

        let head = document.head;
        for (const key of dynamic_css.value) {
            let style_tag = document.createElement("style");

            style_tag.id = key;
            style_tag.innerHTML = css[key];
            
            head.appendChild(style_tag);
        }
    }
});

onBeforeUnmount(() => {
    let head = document.head;
    for (const key of dynamic_css.value) {
        let style_tag = document.getElementById(key);
        if (style_tag) {
            head.removeChild(style_tag);
        }
    }
});
</script>

<template>
    <div class="main" ref="main">
        <div id="content" v-html="contentString"></div>

        <div class="row" v-show="contentString">
            <button class="ml-8" @click="prev_page()">
                prev
                <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24">
                    <path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round"
                        stroke-width="2" d="m15 6l-6 6l6 6" />
                </svg>
            </button>
            <button class="ml-8" @click="next_page()">
                next
                <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24">
                    <path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round"
                        stroke-width="2" d="m9 6l6 6l-6 6" />
                </svg>
            </button>
        </div>
    </div>

</template>

<style>
.main {
    padding: 64px 16px;
}
</style>
