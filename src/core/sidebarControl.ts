import { useSettingStore } from "@/store/settingStore";
import { watch } from "vue";

export default {
    install() {
        const settingStore = useSettingStore();

        watch(
            () => settingStore.show_side_bar,
            (newValue, oldValue) => {
                console.log(newValue, oldValue);
                refreshView(newValue, newValue !== oldValue);
            },
        );
    },
};

export function refreshView(showSidebar: boolean, animation: boolean = false) {
    const side = document.getElementById("side");
    const main = document.getElementById("main");
    const title = document.getElementById("title");

    if (showSidebar) {
        side!.style.display = "block";
        main!.style.marginLeft = 250 + "px";
        title!.style.left = 250 + "px";

        if (animation) {
            side!.animate(
                [
                    { opacity: 0.5, transform: "scale(0.85)" },
                    { opacity: 1, transform: "scale(1)" },
                ],
                {
                    duration: 300,
                    easing: "ease-in-out",
                },
            );
            main!.animate([{ marginLeft: "0" }, { marginLeft: "250px" }], {
                duration: 300,
                easing: "ease-in-out",
            });
            title!.animate(
                [
                    {
                        left: 0,
                    },
                    {
                        left: "250px",
                    },
                ],
                {
                    duration: 300,
                    easing: "ease-in-out",
                },
            );
        }
    } else {
        main!.style.marginLeft = "0";
        title!.style.left = "0";

        if (animation) {
            const animation = side!.animate(
                [
                    { opacity: 1, transform: "scale(1)" },
                    { opacity: 0.5, transform: "scale(0.85)" },
                ],
                {
                    duration: 300,
                    easing: "ease-in-out",
                },
            );
            main!.animate([{ marginLeft: "250px" }, { marginLeft: "0" }], {
                duration: 300,
                easing: "ease-in-out",
            });
            title!.animate(
                [
                    {
                        left: "250px",
                    },
                    {
                        left: 0,
                    },
                ],
                {
                    duration: 300,
                    easing: "ease-in-out",
                },
            );

            animation.onfinish = () => {
                side!.style.display = "none";
            };
        } else {
            side!.style.display = "none";
        }
    }
}
