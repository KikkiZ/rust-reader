import { convertFileSrc } from "@tauri-apps/api/tauri";
import { ref } from "vue";

import appendPath from "@/utils/commonUtils";

console.log("init");

const resource_path = ref("");

// 解析文本内容
function parseToCommonStyle(contents: HTMLCollection, path: string) {
    resource_path.value = path;

    for (const content of contents) {
        traversal(content);
    }
}

function traversal(node: HTMLElement | ChildNode) {
    let element;
    if (node.nodeType === Node.ELEMENT_NODE) {
        element = node as HTMLElement;
    }

    let children = node.childNodes;
    if (children.length === 0 && element !== undefined) {

        if (element.hasAttribute("src")) {
            element = element as HTMLImageElement;

            let target = element.src.substring(7);
            target = appendPath(resource_path.value, target);

            element.src = convertFileSrc(target);
        } else if (element.hasAttribute("href")) {
            element = element as HTMLAnchorElement;

            let target = element.href.substring(7);
            target = appendPath(resource_path.value, target);

            element.href = convertFileSrc(target);
        } else if (element.hasAttribute("xlink:href")) {
            element = element as unknown as SVGImageElement;

            let target = element.href.baseVal.substring(7);
            target = appendPath(resource_path.value, target);

            element.href.baseVal = convertFileSrc(target);
        }

        return;
    }

    for (const current of children) {
        traversal(current);
    }
}

export default parseToCommonStyle;

// TODO: 根据传入参数自动初始化样式
// TODO: 将该解析器重构为一个类