import { createApp } from "vue";
import "./style.scss";
import App from "./App.vue";

import initSync from "./lib/mneq";

initSync();
createApp(App).mount("#app");
