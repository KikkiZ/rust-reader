import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";

import appendPath from "@/utils/commonUtils";

// TODO: 将解析的内容进行重构
// TODO: 完善样式解析

class Parser {
    private resource_path: string;
    private parseType: ParseType;
    private cssNames: string[] = [];

    private index: number = 0;
    private contents: ParsedNode[] = [];

    constructor(path: string, parseType: ParseType) {
        this.resource_path = path;
        this.parseType = parseType;

        this.init();
    }

    // 初始化 Parser, 默认为私有函数
    private async init() {
        console.log("init");

        const result: string = await invoke("get_css");
        const { success, css } = JSON.parse(result);

        let head = document.head;

        // 添加样式
        if (success) {
            this.cssNames = Object.keys(css);

            for (const key of this.cssNames) {
                let styleTag = document.createElement("style");

                styleTag.id = key;
                styleTag.innerHTML = css[key];

                head.appendChild(styleTag);
            }
        }

        // TODO: 添加 Optimize 样式, css 文件已添加, 需要在此处添加样式
        document.getElementById("content")!.classList.add("optimize-content");
    }

    // 文本内容解析
    public contentParse() {
        switch (this.parseType) {
            case ParseType.Native:
                this.parseInNativeMode();
                break;

            case ParseType.Optimize:
                // 重新初始化参数
                this.index = 0;
                this.contents = [];

                this.parseInNativeMode(); // 为了将原始的 DOM 中的图像链接转换为 tauri 的文件路径
                this.parseInOptimizeMode();
                break;
        }
    }

    // 释放相关资源, 在 vue 组件生命周期结束时调用
    public release() {
        console.log("release");

        // 删除样式
        let head = document.head;
        for (const key of this.cssNames) {
            let styleTag = document.getElementById(key);
            if (styleTag) {
                head.removeChild(styleTag);
            }
        }
        document
            .getElementById("content")!
            .classList.remove("optimize-content");
    }

    // 解析为原始样式
    private parseInNativeMode() {
        let contents = document.getElementById("content")?.children;

        if (contents === undefined) return;

        for (const content of contents) {
            this.traversal(content);
        }
    }

    private traversal(node: HTMLElement | ChildNode) {
        let element;
        if (node.nodeType === Node.ELEMENT_NODE) {
            element = node as HTMLElement;
        }

        let children = node.childNodes;
        if (children.length === 0 && element !== undefined) {
            if (element.hasAttribute("src")) {
                element = element as HTMLImageElement;

                let target = element.src.substring(7);
                target = appendPath(this.resource_path, target);

                element.src = convertFileSrc(target);
            } else if (element.hasAttribute("href")) {
                element = element as HTMLAnchorElement;

                let target = element.href.substring(7);
                target = appendPath(this.resource_path, target);

                element.href = convertFileSrc(target);
            } else if (element.hasAttribute("xlink:href")) {
                element = element as unknown as SVGImageElement;

                let target = element.href.baseVal.substring(7);
                target = appendPath(this.resource_path, target);

                element.href.baseVal = convertFileSrc(target);
            }

            return;
        }

        for (const current of children) {
            this.traversal(current);
        }
    }

    // TODO: 优化模式
    private parseInOptimizeMode() {
        const content = document.getElementById("content");
        const contents = content?.children;

        if (contents === undefined) return;

        for (const content of contents) {
            console.log("[0] root element");
            this.loop(content, 1);
            this.index += 1;
        }

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

                const node = document.createElement(item.type);
                node.textContent = item.content;
                // node.setAttribute("v-slide-in", "");

                content?.appendChild(node);
            }); // 将解析结果转为 DOM

        for (const index in this.contents) {
            console.log(index, ": ", this.contents[index]);
        }
        console.log(this.contents);
    }

    private loop(node: HTMLElement | ChildNode, depth: number) {
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

            let msg = "[" + depth + "] ";

            if (current.nodeType === Node.TEXT_NODE) {
                const str = current.textContent?.trim();
                msg += this.type(current.nodeType) + ": " + str;

                // 当前节点可能为 depth 为1的 TEXT_NODE,
                // 此时该节点对应的 ParsedNode 未初始化
                if (this.contents[this.index]) {
                    this.contents[this.index].append(str);
                }
            }

            if (current.nodeType === Node.ELEMENT_NODE) {
                const node = current as HTMLElement;
                msg +=
                    this.type(current.nodeType) +
                    ": " +
                    node.tagName +
                    "; Display: " +
                    this.diplay(node) +
                    "; Class: " +
                    node.classList;
            }

            this.format(depth, msg);
            this.loop(current, depth + 1);
        }
    }

    private format(times: number, str: string) {
        let msg = "";
        for (let i = 0; i < times; i++) {
            msg += "|   ";
        }
        msg += str;
        console.log(msg);
    }

    private type(nodeType: number) {
        switch (nodeType) {
            case 1:
                return "ElementNode";
            case 2:
                return "AttributeNode";
            case 3:
                return "TextNode";
            case 4:
                return "CDATANode";
            case 5:
                return "EntityReferenceNode";
            case 6:
                return "EntityNode";
            case 7:
                return "ProcessingInstructionNode";
            case 8:
                return "CommentNode";
            case 9:
                return "DocumentNode";
            case 10:
                return "DocumentTypeNode";
            default:
                return "UnknownNode";
        }
    }

    private diplay(node: HTMLElement): string {
        return window.getComputedStyle(node).getPropertyValue("display");
    }
}

enum ParseType {
    Native, // 原始样式
    Optimize, // 美化样式
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
    private INLINE_ELEMENTS = [
        "a",
        "b",
        "i",
        "strong",
        "em",
        "code",
        "span",
        "img",
        "sub",
        "sup",
        "small",
        "big",
        "mark",
        "del",
        "ins",
        "u",
        "s",
        "q",
        "cite",
        "kbd",
        "var",
        "samp",
        "time",
        "ruby",
        "rt",
        "rp",
        "abbr",
        "data",
        "time",
        "dfn",
        "address",
        "ins",
        "del",
        "small",
        "sub",
        "sup",
    ];

    public displayType() {
        const tag = this.type.toLowerCase();

        if (this.BLOCK_ELEMENTS.includes(tag)) return "BLOCK";
        if (this.INLINE_ELEMENTS.includes(tag)) return "INLINE";
        return "UNKNOWN";
    }
}

export { Parser, ParseType };
