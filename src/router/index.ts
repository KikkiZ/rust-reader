import { createRouter, createWebHistory } from "vue-router"

import Read from "@/views/page/Read.vue";
import BookList from "@/views/sidebar/BookList.vue";
import Index from "@/views/page/Index.vue";
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
