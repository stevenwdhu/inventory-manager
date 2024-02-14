import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import router from "./router";
import "element-plus/theme-chalk/index.css";
import { DELIVERY_LOG, INVENTORY, RECEIVER_LIST } from "./const";

const app = createApp(App);
app.use(router).mount("#app");

app.directive("focus", {
  mounted: (el) => {
    el.querySelector("input").focus();
  },
});

const setDefault = (key: string, value: string) => {
  if (localStorage.getItem(key)) return;
  localStorage.setItem(key, value);
};

setDefault(INVENTORY, "./data/inventory.json");
setDefault(DELIVERY_LOG, "./data/delivery_log.json");
setDefault(RECEIVER_LIST, "./data/receiver_list.json");
