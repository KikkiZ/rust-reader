import { useConfigStore } from "@/store/configStore";
import { watch } from "vue";

export default {
    install() {
        const configStore = useConfigStore();

        watch(
            () => configStore.setting.sidebar,
            (newValue, oldValue) => {
                refreshView(newValue, newValue !== oldValue);
            },
        );
    },
};

export function refreshView(showSidebar: boolean, animation: boolean = false) {
    const side = document.getElementById("side");
    const main = document.getElementById("main");

    if (showSidebar) {
        side!.style.display = "block";

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
            main!.animate([{ marginLeft: "-250px" }, { marginLeft: "0" }], {
                duration: 300,
                easing: "ease-in-out",
            });
        }

    } else {

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
            main!.animate([{ marginLeft: "0" }, { marginLeft: "-250px" }], {
                duration: 300,
                easing: "ease-in-out",
            });

            animation.onfinish = () => {
                side!.style.display = "none";
            };

        } else {
            side!.style.display = "none";
        }
    }
}
