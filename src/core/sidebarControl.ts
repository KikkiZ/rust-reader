import { useSettingStore } from "@/store/settingStore";
import { watch } from "vue";

export default {
    install() {
        const settingStore = useSettingStore();

        watch(
            () => settingStore.show_side_bar,
            new_value => {
                refreshView(new_value);
            },
        );
    }
}

export function refreshView(flag: boolean) {
    const side = document.getElementById("side");
    const main = document.getElementById("main");

    if (flag) {
        side!.style.display = "block";
        main!.style.marginLeft = 250 + "px";
    } else {
        side!.style.display = "none";
        main!.style.marginLeft = "0";
    }
}

