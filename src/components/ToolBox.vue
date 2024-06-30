<script setup></script>

<template>
    <div class="tool-box">
        <div class="item">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="1em"
                height="1em"
                viewBox="0 0 24 24">
                <g
                    fill="none"
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="1.5">
                    <path
                        d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 0 0-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 0 0-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 0 0-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 0 0-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 0 0 1.066-2.573c-.94-1.543.826-3.31 2.37-2.37c1 .608 2.296.07 2.572-1.065" />
                    <path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0-6 0" />
                </g>
            </svg>
            <p>设置</p>
        </div>
        <div class="item">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="1em"
                height="1em"
                viewBox="0 0 24 24">
                <path
                    fill="none"
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="1.5"
                    d="M12 19H5a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h4l3 3h7a2 2 0 0 1 2 2v3.5M19 22v-6m3 3l-3-3l-3 3" />
            </svg>
            <p>上传</p>
        </div>
    </div>
</template>

<style scoped>
.tool-box {
    position: sticky;
    bottom: 0;
    display: flex;
    gap: 4px;

    padding: 4px;
    border-radius: 6px;

    background-color: rgba(255, 255, 255, 0.35);
    -webkit-backdrop-filter: blur(5px);
    backdrop-filter: blur(5px);
}

.item {
    width: 40px;
    height: 54px;
    border-radius: 4px;

    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
}

.item:hover {
    background-color: rgba(255, 255, 255, 0.55);
}

.item > svg {
    width: 32px;
    height: 32px;
    color: #353535;
}

.item > p {
    font-size: 0;
    line-height: 0;
    padding: 0;
    margin: 0;
    margin-top: 4px;
    font-weight: 600;
    color: #353535;
}

.item:hover > p {
    font-size: 14px;
    line-height: 14px;
}
</style>
