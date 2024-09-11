import { createApp } from "vue";
import "./assets/css/global.css";
import "./assets/css/content.css";
import App from "./App.vue";
import router from "./router";
import pinia from "./store";
import sidebarControl from "./core/sidebarControl";
import slideIn from "./utils/vSlideIn";
import split from "./utils/vSplit";

const app = createApp(App);

app.use(pinia);
app.use(router);
app.use(sidebarControl);
app.directive("slide-in", slideIn);
app.directive("split", split);
app.mount("#app");
