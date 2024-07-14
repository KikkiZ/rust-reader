import { convertFileSrc } from "@tauri-apps/api/tauri";

import appendPath from "@/utils/commonUtils";

// TODO: 将解析的内容进行重构
// TODO: 完善样式解析

class Parser {
    private resource_path: string;

    private index: number = 0;
    private contents: ParsedNode[] = [];

    constructor(path: string) {
        this.resource_path = path;

        this.init();
    }

    // 初始化 Parser, 默认为私有函数
    private async init() {
        console.log("init");

        document.getElementById("content")!.classList.add("optimize-content");
        document.getElementById("main")!.style.padding = "8px";
    }

    // 文本内容解析
    public contentParse() {
        // 重新初始化参数
        this.index = 0;
        this.contents = [];

        this.parse(); // 为了将原始的 DOM 中的图像链接转换为 tauri 的文件路径
        this.optimize();
    }

    // 释放相关资源, 在 vue 组件生命周期结束时调用
    public release() {
        console.log("release");

        document.getElementById("main")!.style.padding = "";
        document
            .getElementById("content")!
            .classList.remove("optimize-content");
    }

    // 解析为原始样式
    private parse(): void;
    private parse(node: HTMLElement | ChildNode): void;
    private parse(node?: HTMLElement | ChildNode) {
        if (node === undefined) {
            let contents = document.getElementById("content")?.children;

            if (contents === undefined) return;

            for (const content of contents) {
                this.parse(content);
            }
        } else {
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
                this.parse(current);
            }

        }
    }

    // TODO: 优化模式
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
                node.textContent = item.content;
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

            this.traversal(current, depth + 1);
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
