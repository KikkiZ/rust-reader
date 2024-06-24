import NotificationItem from "@/components/NotificationItem.vue";
import Notification from "@/entity/notification";
import { createApp } from "vue";

export function notify(notification: Notification, duration: number = 5000) {
    const panel = document.getElementById("notification-panel")!;
    const item = document.createElement("div");

    const notify = createApp(NotificationItem, {
        notice: notification,
        duration: duration,
    });

    notify.mount(item);
    panel.appendChild(item);
    const offset = item.offsetHeight;

    for (let index = 0; index < panel.children.length; index++) {
        if (index === panel.children.length - 1) {
            break;
        }

        panel.children[index].animate(
            [
                { transform: `translateY(${offset / 2}px)` },
                { transform: `translateY(0)` },
            ],
            { duration: 300, easing: "ease-in-out" },
        );
    }

    item.animate(
        [{ transform: "translateX(125px)" }, { transform: "translateX(0)" }],
        { duration: 600, easing: "ease-in-out" },
    );

    cleanEmptyItem(panel);
}

function cleanEmptyItem(panel: HTMLElement) {
    const children = panel.children;
    for (const item of children) {
        if (item.children.length == 0) {
            item.remove();
        }
    }
}
