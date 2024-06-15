<script setup lang="ts">
/**
 * @component ToggleButton
 * @description 这是一个 Toggle 按钮
 *
 * @props {Number} [width:40] - 按钮宽度, 默认为 40px
 * @props {Number} [height:25] - 按钮高度, 默认为 25px
 * @props {Boolean} [value: false] - 按钮初始状态, 默认为 false
 *
 * @emits {event: toggle-value} - 当按钮的值切换时, 返回切换后的值
 *
 * @example
 * <ToggleButton :width=100 :height=50 @toggle-value="handle"/>
 * <ToggleButton :value=true @toggle-value="handle" />
 */
import { onMounted, ref } from "vue";

const button = ref();
const round = ref();
const toggleValue = ref(false);

function toggle() {
    toggleValue.value = !toggleValue.value;
    styleTransition();
    emit("toggle-value", toggleValue.value);
}

function styleTransition() {
    if (toggleValue.value) {
        button.value.style.flexDirection = "row-reverse";
        button.value.style.backgroundColor = "#34c659";
    } else {
        button.value.style.flexDirection = "row";
        button.value.style.backgroundColor = "#e9e9e9";
    }
}

const props = defineProps({
    width: { type: Number, default: 40 },
    height: { type: Number, default: 25 },
    value: { type: Boolean, default: false },
});

const emit = defineEmits(["toggle-value"]);

onMounted(() => {
    button.value.style.width = props.width + "px";
    button.value.style.height = props.height + "px";

    const minValue = props.width < props.height ? props.width : props.height;
    button.value.style.borderRadius = minValue / 2 + "px";

    round.value.style.width = minValue - 6 + "px";
    round.value.style.height = minValue - 6 + "px";

    toggleValue.value = props.value;
    styleTransition();
});
</script>

<template>
    <div class="toggle-button" ref="button" @click="toggle">
        <div class="round-button" ref="round"></div>
    </div>
</template>

<style scoped>
.toggle-button {
    background-color: #e9e9e9;
    display: flex;
    align-items: center;
}

.round-button {
    background-color: #ffffff;
    border-radius: 50%;
    margin: 0 3px;
}
</style>
