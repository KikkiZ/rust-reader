<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { nextTick, onBeforeUnmount, onMounted, ref, watch } from "vue";

import { appendPath, randomString } from "@/utils/commonUtils";
import { useAppStateStore } from "@/store/appStateStore";
import { useConfigStore } from "@/store/configStore";
import { Parser } from "@/core/contentParser";
import { refreshView } from "@/core/sidebarControl";
import { notify } from "@/core/notifyService";
import { BookMark } from "@/entity/bookMark";

const configStore = useConfigStore();
const appStateStore = useAppStateStore();

const main = ref();
const contentString = ref("");
const contentParser = ref<Parser>();
const archiveContent = ref<HTMLElement>();

watch(
    () => appStateStore.current_chapter,
    async (newValue) => {
        const result: string = await invoke("jump_to_chapter", {
            chapter: newValue,
        });
        const { content, success, msg } = JSON.parse(result);

        if (success) {
            contentString.value = content;
        } else {
            notify(msg);
        }
    },
);

watch(
    () => contentString.value,
    () => {
        nextTick().then(() => {
            contentParser.value?.contentParse();
            archiveContent.value = document
                .getElementById("content")!
                .cloneNode(true) as HTMLElement;

            // 滚动到顶部
            const start = main.value.scrollTop;
            const change = -start;
            const startTime = performance.now();

            function animateScroll(currentTime: number) {
                const elapsed = currentTime - startTime;
                const progress = Math.min(elapsed / 300, 1); // 动画执行的进度
                main.value.scrollTop = start + change * progress;

                if (progress < 1) {
                    requestAnimationFrame(animateScroll);
                }
            }

            // 该函数由浏览器提供, 用于创建动画帧
            // 该函数接收一个回调函数作为参数, 在回调函数中
            // 更新元素的变化, 并可以根据时间戳来判断动画是否完成
            requestAnimationFrame(animateScroll);

            refreshBookMark();
        });
    },
    { deep: true },
);

async function prevPage() {
    const result: string = await invoke("prev_page");
    const { content, success, msg } = JSON.parse(result);

    if (success) {
        contentString.value = content;
        appStateStore.current_chapter -= 1;
    } else {
        notify(msg);
    }
}

async function nextPage() {
    const result: string = await invoke("next_page");
    const { content, success, msg } = JSON.parse(result);

    if (success) {
        contentString.value = content;
        appStateStore.current_chapter += 1;
    } else {
        notify(msg);
    }
}

async function openBook(id: string) {
    const result: string = await invoke("open_book", { id: id });
    const { content, success, msg } = JSON.parse(result);

    if (success) {
        contentString.value = content;
        appStateStore.current_chapter = 0;
    } else {
        notify(msg);
    }
}

// 书签逻辑:
// 页面解析(ContentParser)完之后记录解析完的数据,
// 获取书签列表, 依次渲染书签, 并在渲染每个书签时添加删除按钮.
// 删除书签逻辑:
// 发送请求到后台, 从数据库中删除数据,
// 将解析完成的数据替换当前内容, 重新执行书签逻辑 
const markId = new Map<number, string[]>();

// 新增书签
async function addBookMark(event: MouseEvent) {
    const target = event.target! as HTMLElement;
    const paragraph = parseInt(target.id);
    const length = target.textContent?.length!;

    const mark: BookMark = {
        book_id: appStateStore.current_book_id,
        mark_id: 0,
        start_position: {
            chapter: appStateStore.current_chapter,
            paragraph: paragraph,
            offset: 0,
        },
        end_position: {
            chapter: appStateStore.current_chapter,
            paragraph: paragraph,
            offset: length,
        },
        create_time: 0,
    };

    // 处理返回值
    const result: string = await invoke("add_bookmark", {
        data: JSON.stringify(mark),
    });
    const { success, msg } = JSON.parse(result);

    if (!success) {
        notify(msg);
    } else {
        highlight(mark);
    }
}

// 刷新书签
// TODO: 刷新完书签后需要添加监听器 -> 处理已有的监听器
async function refreshBookMark() {
    // document.querySelectorAll(".delete").forEach((item) => {
    //     item.remove();
    // });

    markId.clear();
    const result: string = await invoke("get_chapter_mark_list", {
        id: appStateStore.current_book_id,
        chapter: appStateStore.current_chapter,
    });
    const { success, list, msg } = JSON.parse(result);

    if (success) {
        // 渲染书签
        for (const mark of list) {
            highlight(mark);
        }
    } else {
        notify(msg);
    }

    // TODO: “新增按钮” 的逻辑添加到 highlight 函数中
    // document.querySelectorAll(".bookmark").forEach((item) => {
    //     const deleteButton = document.createElement("button");
    //     deleteButton.classList.add("delete");
    //     deleteButton.textContent = "删除";

    //     deleteButton.addEventListener("click", () => {
    //         if (contain(item.id)) {
    //             const id = getKey(item.id);
    //             markId.delete(id);
    //             // TODO: 处理返回值
    //             invoke("delete_mark", { id: id });

    //             const newContent = archiveContent.value!;
    //             const parent = main.value!;
    //             const oldContent = document.getElementById("content")!;

    //             parent.appendChild(newContent);
    //             parent.replaceChild(newContent, oldContent);

    //             refreshBookMark();
    //         }
    //     });
    //     // 阻止双击按钮向父元素冒泡
    //     deleteButton.addEventListener("dblclick", (event) => {
    //         event.stopPropagation();
    //     });

    //     item.appendChild(deleteButton);
    // });

    console.log(markId);
}

// 渲染书签
function highlight(mark: BookMark) {
    if (mark.start_position.paragraph === mark.end_position.paragraph) {
        const elememt = document.getElementById(
            mark.start_position.paragraph.toString(),
        )!;

        if (!markId.has(mark.mark_id)) {
            markId.set(mark.mark_id, []);
        }

        const text = elememt.textContent!;
        const before = text.substring(0, mark.start_position.offset);
        const highlight = text.substring(
            mark.start_position.offset,
            mark.end_position.offset,
        );
        const after = text.substring(mark.end_position.offset);

        const highlightNode = document.createElement("span");
        highlightNode.textContent = highlight;
        highlightNode.classList.add("bookmark");
        highlightNode.id = randomString(10);

        markId.get(mark.mark_id)!.push(highlightNode.id);

        const deleteButton = document.createElement("button");
        deleteButton.classList.add("delete");
        deleteButton.textContent = "删除";

        deleteButton.addEventListener("click", () => {
            // 处理删除逻辑, 需要重新给 content 添加监听器
            if (contain(highlightNode.id)) {
                console.log(highlightNode.id) 
                // FIX: 存在隐式的逻辑问题
                const id = getKey(highlightNode.id);
                markId.delete(id);
                // TODO: 处理返回值
                invoke("delete_mark", { id: id });

                const newContent = archiveContent.value!;
                const parent = main.value!;
                const oldContent = document.getElementById("content")!;

                oldContent.removeEventListener("dblclick", addBookMark);
                newContent.addEventListener("dblclick", addBookMark);

                parent.appendChild(newContent);
                parent.replaceChild(newContent, oldContent);

                refreshBookMark();
            }
        });
        // 阻止双击按钮向父元素冒泡
        deleteButton.addEventListener("dblclick", (event) => {
            event.stopPropagation();
        });

        highlightNode.appendChild(deleteButton);

        elememt.innerHTML = "";
        elememt.appendChild(document.createTextNode(before));
        elememt.appendChild(highlightNode);
        elememt.appendChild(document.createTextNode(after));

    } else {
        bookMarkSplit(mark).forEach((item) => {
            highlight(item);
        });
    }
}

// 书签分割
function bookMarkSplit(mark: BookMark): BookMark[] {
    if (mark.start_position.paragraph === mark.end_position.paragraph) {
        return [];
    }

    let arr = [];
    arr.push({
        book_id: "",
        mark_id: mark.mark_id,
        start_position: {
            chapter: appStateStore.current_chapter,
            paragraph: mark.start_position.paragraph,
            offset: mark.start_position.offset,
        },
        end_position: {
            chapter: appStateStore.current_chapter,
            paragraph: mark.start_position.paragraph,
            offset: document.getElementById(
                mark.start_position.paragraph.toString(),
            )!.textContent?.length!,
        },
        create_time: 0,
    });

    for (
        let index = mark.start_position.paragraph + 1;
        index < mark.end_position.paragraph;
        index++
    ) {
        arr.push({
            book_id: "",
            mark_id: mark.mark_id,
            start_position: {
                chapter: appStateStore.current_chapter,
                paragraph: index,
                offset: 0,
            },
            end_position: {
                chapter: appStateStore.current_chapter,
                paragraph: index,
                offset: document.getElementById(index.toString())!.textContent
                    ?.length!,
            },
            create_time: 0,
        });
    }

    arr.push({
        book_id: "",
        mark_id: mark.mark_id,
        start_position: {
            chapter: appStateStore.current_chapter,
            paragraph: mark.end_position.paragraph,
            offset: 0,
        },
        end_position: {
            chapter: appStateStore.current_chapter,
            paragraph: mark.end_position.paragraph,
            offset: mark.end_position.offset,
        },
        create_time: 0,
    });

    return arr;
}

function contain(value: string): boolean {
    let result = false;
    markId.forEach((values, _) => {
        if (values.includes(value)) {
            result = true;
        }
    });

    return result;
}

function getKey(value: string): number {
    let result = -1;
    markId.forEach((values, key) => {
        if (values.includes(value)) {
            result = key;
        }
    });

    return result;
}

onMounted(async () => {
    refreshView(configStore.setting.sidebar);

    // TODO: 调整接口用法
    const path: string = await invoke("get_resource_path");
    const resourcePath = appendPath(path, appStateStore.current_book_id);

    openBook(appStateStore.current_book_id);

    contentParser.value = new Parser(resourcePath);

    document
        .getElementById("content")!
        .addEventListener("dblclick", addBookMark);

    // const mark: BookMark = {
    //     book_id: appStateStore.current_book_id,
    //     mark_id: 0,
    //     start_position: {
    //         chapter: appStateStore.current_chapter,
    //         paragraph: 12,
    //         offset: 5,
    //     },
    //     end_position: {
    //         chapter: appStateStore.current_chapter,
    //         paragraph: 14,
    //         offset: 9,
    //     },
    //     create_time: 0,
    // };
    // console.log(mark);
    // invoke("add_bookmark", {
    //     data: JSON.stringify(mark),
    // });
});

onBeforeUnmount(() => {
    document
        .getElementById("content")!
        .removeEventListener("dblclick", addBookMark);
    contentParser.value?.release();
});
</script>

<template>
    <div class="main" ref="main">
        <div id="content" v-html="contentString"></div>

        <div class="row" v-show="contentString">
            <button class="ml-8" @click="prevPage()">
                prev
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="1em"
                    height="1em"
                    viewBox="0 0 24 24">
                    <path
                        fill="none"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="m15 6l-6 6l6 6" />
                </svg>
            </button>
            <button class="ml-8" @click="nextPage()">
                next
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="1em"
                    height="1em"
                    viewBox="0 0 24 24">
                    <path
                        fill="none"
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="m9 6l6 6l-6 6" />
                </svg>
            </button>
        </div>
    </div>
</template>

<style scoped></style>
