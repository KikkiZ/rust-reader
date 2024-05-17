import { createRouter, createWebHistory } from "vue-router"

import Read from "@/components/page/Read.vue";
import BookList from "@/components/sidebar/BookList.vue";
import Index from "@/components/page/Index.vue";
import Catalog from "@/components/sidebar/Catalog.vue";

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
