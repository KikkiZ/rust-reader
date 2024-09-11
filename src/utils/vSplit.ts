import { Directive, ref } from "vue";
import pinia from "@/store";
import { useAppStateStore } from "@/store/appStateStore";

const appStateStore = useAppStateStore(pinia);

const observer = new MutationObserver((mutationList) => {
    for (const mutation of mutationList) {
        if (
            mutation.type === "attributes" &&
            mutation.attributeName === "style"
        ) {
            const node = mutation.target as HTMLElement;
            injectSplit(node.parentElement!, option.value!);
        }
    }
});

interface Option {
    split_width: number;
    direction: string; // vertical | horizontal
}

function filterToOption(obj: any): Option {
    const attrs: Map<keyof Option, string> = new Map();
    attrs.set("split_width", "number");
    attrs.set("direction", "string");

    let option: Option = {
        split_width: 8,
        direction: "vertical",
    };

    if (obj === undefined) return option;

    attrs.forEach((value, key) => {
        if (key in obj && typeof obj[key] === value) {
            (option as any)[key] = obj[key];
        }
    });

    return option;
}

function splitBuilder(sibling: HTMLElement, option: Option): HTMLElement {
    const split = document.createElement("div");

    split.classList.add("split");
    split.classList.add(option.direction);

    option.direction === "vertical"
        ? (split.style.width = `${option.split_width}px`)
        : (split.style.height = `${option.split_width}px`);

    split.onmousedown = () => {
        console.log("mouse down");
        document.onmousemove = createMouseMoveHandler(sibling);
    };

    split.onmouseup = () => {
        console.log("mouse up");
        document.onmousemove = null;
    };

    split.ondragstart = () => false;

    return split;
}

function createMouseMoveHandler(
    sibling: HTMLElement,
): (event: MouseEvent) => void {
    return (event: MouseEvent) => {
        appStateStore.sidebar_width = event.pageX;

        const gap = window
            .getComputedStyle(document.documentElement)
            .getPropertyValue("--gap");

        sibling.style.width = `calc(${appStateStore.sidebar_width}px - ${gap})`;
        sibling.style.flex = `0 0 calc(${appStateStore.sidebar_width}px - ${gap})`;
    };
}

function isDisplay(element: HTMLElement): boolean {
    return window.getComputedStyle(element).display !== "none";
}

function removeSplit(element: HTMLElement) {
    element.childNodes.forEach((child) => {
        let node = child as HTMLElement;
        if (node.classList.contains("split")) {
            node.remove();
        }
    });
}

function injectSplit(element: HTMLElement, option: Option) {
    removeSplit(element);
    if (element.childNodes.length >= 2) {
        element.childNodes.forEach((child) => {
            let node = child as HTMLElement;
            if (
                !node.classList.contains("split") &&
                node.nextSibling !== null &&
                isDisplay(node)
            ) {
                const split = splitBuilder(child as HTMLElement, option);
                element.insertBefore(split, node.nextSibling);
            }
        });
    }
}

const option = ref<Option>();
const split: Directive = {
    mounted(element: HTMLElement, bounding: any) {
        option.value = filterToOption(bounding.value);
        injectSplit(element, option.value);

        observer.observe(element, { attributes: true, subtree: true });
    },

    unmounted(_element: HTMLElement) {
        observer.disconnect();
    },
};

export default split;
