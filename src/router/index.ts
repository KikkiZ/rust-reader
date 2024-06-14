import { createRouter, createWebHistory } from "vue-router"

import Read from "@/views/page/Read.vue";
import Index from "@/views/page/Index.vue";
import Detail from "@/views/page/Detail.vue";
import BookList from "@/views/sidebar/BookList.vue";
import Catalog from "@/views/sidebar/Catalog.vue";

const routes = [
    {
        path: "/",
        redirect: "/list"
    },
    {
        path: "/list",
        components: {
            side: BookList,
            main: Index,
        },
    },
    {
        path: "/detail",
        components: {
            side: BookList,
            main: Detail,
        },
    },
    {
        path: "/read",
        components: {
            side: Catalog,
            main: Read,
        },
    }
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;
