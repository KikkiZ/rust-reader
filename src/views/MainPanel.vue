<script setup lang="ts"></script>

<template>
    <div id="main-panel">
        <div id="side">
            <RouterView name="side" />
        </div>
        <div id="main">
            <RouterView name="main" />
        </div>
    </div>
</template>

<style scoped></style>
