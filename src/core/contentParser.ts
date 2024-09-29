import { convertFileSrc, invoke } from "@tauri-apps/api/core";

import { appendPath } from "@/utils/commonUtils";

// TODO: 完善样式解析

class Parser {
    private resource_path: string;
    private css_names: string[] = [];

    private index: number = 0;
    private contents: ParsedNode[] = [];

    constructor(path: string) {
        this.resource_path = path;

        this.init();
    }

    // 初始化 Parser, 默认为私有函数
    private async init() {
        console.log("init");

        const result: string = await invoke("get_css");
        const { success, css } = JSON.parse(result);

        // 添加样式
        if (success) {
            this.css_names = Object.keys(css);

            let head = document.head;
            for (const key of this.css_names) {
                let style_tag = document.createElement("style");

                style_tag.id = key;
                style_tag.innerHTML = css[key];

                head.appendChild(style_tag);
            }
        }

        document.getElementById("content")!.classList.add("optimize-content");
        document.getElementById("main")!.style.padding = "8px";
    }

    // 文本内容解析
    public contentParse() {
        // 重新初始化参数
        this.index = 0;
        this.contents = [];

        this.optimize();
    }

    // 释放相关资源, 在 vue 组件生命周期结束时调用
    public release() {
        console.log("release");

        let head = document.head;
        for (const key of this.css_names) {
            let style_tag = document.getElementById(key);
            if (style_tag) {
                head.removeChild(style_tag);
            }
        }

        document.getElementById("main")!.style.padding = "";
        document
            .getElementById("content")!
            .classList.remove("optimize-content");
    }

    private optimize() {
        const content = document.getElementById("content");
        const contents = content?.children;

        if (contents === undefined) return;

        for (const content of contents) {
            this.traversal(content, 1);
            this.index += 1;
        }

        let count = 0;
        content!.innerHTML = ""; // 清空内容
        this.contents
            .filter((item) => item !== undefined && item !== null) // 过滤掉未定义的一些空节点
            .map((item) => {
                if (item.type === undefined) {
                    item.type = "p";
                } else {
                    item.type = item.type.toLowerCase();
                }

                return item;
            }) // 处理节点内容
            .forEach((item) => {
                if (item.type === "aside") return;
                if (item.type === "small") {
                    const text = content?.lastChild?.textContent;
                    if (text) {
                        content.lastChild.textContent += item.content;
                        return;
                    }
                }

                // TODO: 添加样式
                const node = document.createElement(item.type);
                node.innerHTML = item.content;
                node.setAttribute("id", count.toString());
                if (!item.isBlock()) {
                    node.classList.add("block");
                }

                content?.appendChild(node);
                count += 1;
            }); // 将解析结果转为 DOM
    }

    private traversal(node: HTMLElement | ChildNode, depth: number) {
        const children = node.childNodes;

        // 当没有子节点时，结束递归
        if (children.length === 0) return;

        for (const current of children) {
            // 当深度为1且不为空的文本节点时, 初始化一个节点
            // 当 if 语句的条件为空字符串时, 条件表达式为 false
            if (depth === 1 && current.textContent?.trim()) {
                this.index += 1;

                // 初始化 ParsedNode
                const node = current as HTMLElement;
                const parsedNode = new ParsedNode(node.tagName, "");
                this.contents[this.index] = parsedNode;
            }

            if (current.nodeType === Node.TEXT_NODE) {
                const str = current.textContent?.trim();
                // 当前节点可能为 depth 为1的 TEXT_NODE,
                // 此时该节点对应的 ParsedNode 未初始化
                if (this.contents[this.index]) {
                    this.contents[this.index].append(str);
                }
            }

            if (
                this.isImageNode(current as HTMLElement) &&
                this.contents[this.index]
            ) {
                this.convertSrcLink(current as HTMLElement);
                const imageNode = (current as HTMLElement).outerHTML;
                this.contents[this.index].append(imageNode);
            }

            this.traversal(current, depth + 1);
        }
    }

    private isImageNode(node: HTMLElement): boolean {
        if (node === undefined || node === null) return false;
        if (!(node instanceof HTMLElement)) return false;
        if (
            !node.hasAttribute("src") &&
            !node.hasAttribute("href") &&
            !node.hasAttribute("xlink:href")
        ) {
            return false;
        }

        let target;
        if (node.hasAttribute("src")) {
            const ele = node as HTMLImageElement;
            target = ele.src.substring(7);
        } else if (node.hasAttribute("href")) {
            const ele = node as HTMLAnchorElement;
            target = ele.href.substring(7);
        } else if (node.hasAttribute("xlink:href")) {
            const ele = node as unknown as SVGImageElement;
            target = ele.href.baseVal.substring(7);
        } else {
            return false;
        }

        const suffix = [".png", ".jpg", ".jpeg", ".gif"];
        if (!suffix.some((item) => target.endsWith(item))) return false;

        return true;
    }

    private convertSrcLink(node: HTMLElement) {
        if (node.hasAttribute("src")) {
            const element = node as HTMLImageElement;

            let target = element.src.substring(7);
            target = appendPath(this.resource_path, target);

            element.src = convertFileSrc(target);
            // 将 alt 属性转为 title 属性, 指针指向 img 标签时会显示详细内容
            element.title = element.alt;
        } else if (node.hasAttribute("href")) {
            const element = node as HTMLAnchorElement;

            let target = element.href.substring(7);
            target = appendPath(this.resource_path, target);

            element.href = convertFileSrc(target);
        } else if (node.hasAttribute("xlink:href")) {
            const element = node as unknown as SVGImageElement;

            let target = element.href.baseVal.substring(7);
            target = appendPath(this.resource_path, target);

            element.href.baseVal = convertFileSrc(target);
        }
    }
}

class ParsedNode {
    public type: string;
    public content: string;
    public style: string[];

    constructor(type: string, content: string, style?: string[]) {
        // TODO: 解析节点类型
        this.type = type;
        this.content = content;

        if (style) {
            this.style = style;
        } else {
            this.style = [];
        }
    }

    public append(content: string | undefined) {
        if (content) {
            this.content += content;
        }
    }

    private BLOCK_ELEMENTS = [
        "div",
        "p",
        "h1",
        "h2",
        "h3",
        "h4",
        "h5",
        "h6",
        "ul",
        "ol",
        "li",
        "table",
        "thead",
        "tbody",
        "tr",
        "th",
        "td",
        "blockquote",
        "pre",
        "dl",
        "dt",
        "dd",
        "hr",
        "br",
        "nav",
        "aside",
    ];
    // private INLINE_ELEMENTS = [
    //     "a",
    //     "b",
    //     "i",
    //     "strong",
    //     "em",
    //     "code",
    //     "span",
    //     "img",
    //     "sub",
    //     "sup",
    //     "small",
    //     "big",
    //     "mark",
    //     "del",
    //     "ins",
    //     "u",
    //     "s",
    //     "q",
    //     "cite",
    //     "kbd",
    //     "var",
    //     "samp",
    //     "time",
    //     "ruby",
    //     "rt",
    //     "rp",
    //     "abbr",
    //     "data",
    //     "time",
    //     "dfn",
    //     "address",
    //     "ins",
    //     "del",
    //     "small",
    //     "sub",
    //     "sup",
    // ];

    // public displayType() {
    //     const tag = this.type.toLowerCase();

    //     if (this.BLOCK_ELEMENTS.includes(tag)) return "BLOCK";
    //     if (this.INLINE_ELEMENTS.includes(tag)) return "INLINE";
    //     return "UNKNOWN";
    // }

    public isBlock(): boolean {
        const tag = this.type.toLowerCase();

        if (this.BLOCK_ELEMENTS.includes(tag)) return true;
        return false;
    }
}

export { Parser };
