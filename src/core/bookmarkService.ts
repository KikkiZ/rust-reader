import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";

import pinia from "@/store";
import { notify } from "@/core/notifyService";
import { BookMark } from "@/entity/bookMark";
import { useAppStateStore } from "@/store/appStateStore";
import { randomString } from "@/utils/commonUtils";

class ZipperMap<K, V> {
    private map: Map<K, V[]>;

    constructor() {
        this.map = new Map();
    }

    public get(key: K): V[] {
        return this.map.get(key) || [];
    }

    public set(key: K, value: V[]) {
        this.map.set(key, value);
    }

    public pushInto(key: K, value: V) {
        if (this.map.has(key)) {
            this.map.get(key)?.push(value);
        } else {
            this.map.set(key, [value]);
        }
    }

    public has(key: K): boolean {
        return this.map.has(key);
    }

    public contain(value: V): boolean {
        let result = false;
        this.map.forEach((values, _) => {
            if (values.includes(value)) {
                result = true;
            }
        });

        return result;
    }

    public getKey(value: V): K | null {
        let result = null;
        this.map.forEach((values, key) => {
            if (values.includes(value)) {
                result = key;
            }
        });

        return result;
    }

    public delete(key: K) {
        this.map.delete(key);
    }

    public clear() {
        this.map.clear();
    }
}

// 书签逻辑:
// 页面解析(ContentParser)完之后记录解析完的数据,
// 获取书签列表, 依次渲染书签, 并在渲染每个书签时添加删除按钮.
// 删除书签逻辑:
// 发送请求到后台, 从数据库中删除数据,
// 将解析完成的数据替换当前内容, 重新执行书签逻辑

const appStateStore = useAppStateStore(pinia);

const archiveContent = ref<HTMLElement>();
const markId = new ZipperMap<number, string>();

// 设置 archiveContent
function setContent(content: HTMLElement) {
    archiveContent.value = content;
}

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
            if (markId.contain(highlightNode.id)) {
                console.log(highlightNode.id) 
                // FIX: 存在隐式的逻辑问题
                const id = markId.getKey(highlightNode.id);
                markId.delete(id!);
                // TODO: 处理返回值
                invoke("delete_mark", { id: id });

                const newContent = archiveContent.value!;
                const oldContent = document.getElementById("content")!;
                const parent = oldContent.parentNode!;

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

export { setContent, addBookMark, refreshBookMark };
