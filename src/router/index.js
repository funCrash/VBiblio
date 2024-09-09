import { createRouter, createWebHistory } from "vue-router";

const routes = [
    {
        path: "/",
        name: "home",
        component: () => import("../views/HomeView.vue"),
    },
    {
        path:"/book/:id",
        name: "bookDetails",
        component: () => import("../views/DetailsView.vue"),
    },
]

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes,
});

export default router