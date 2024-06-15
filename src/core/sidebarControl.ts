import { useSettingStore } from "@/store/settingStore";
import { watch } from "vue";

export default {
    install() {
        const settingStore = useSettingStore();

        watch(
            () => settingStore.show_side_bar,
            (newValue) => {
                refreshView(newValue);
            },
        );
    },
};

export function refreshView(flag: boolean) {
    const side = document.getElementById("side");
    const main = document.getElementById("main");

    if (flag) {
        side!.style.display = "block";
        main!.style.marginLeft = 250 + "px";

        side!.animate([{ left: "-250px" }, { left: "0" }], {
            duration: 300,
            easing: "ease-in-out",
        });
        main!.animate([{ marginLeft: "0" }, { marginLeft: "250px" }], {
            duration: 300,
            easing: "ease-in-out",
        });
    } else {
        // side!.style.display = "none";
        main!.style.marginLeft = "0";

        const animation = side!.animate([{ left: "0" }, { left: "-250px" }], {
            duration: 300,
            easing: "ease-in-out",
        });
        main!.animate([{ marginLeft: "250px" }, { marginLeft: "0" }], {
            duration: 300,
            easing: "ease-in-out",
        });

        animation.onfinish = () => {
            side!.style.display = "none";
        };
    }
}
