import Checkout from "./components/Checkout.vue";
import { createRouter, createWebHashHistory } from "vue-router";
import Inventory from "./components/Inventory.vue";
import DeliveryLog from "./components/DeliveryLog.vue";
import Settings from "./components/Settings.vue";
import ReceiverList from "./components/ReceiverList.vue";

const routes = [
  { path: "/", redirect: "/checkout" },
  { path: "/checkout", component: Checkout },
  { path: "/inventory", component: Inventory },
  { path: "/delivery_log", component: DeliveryLog },
  { path: "/receiver_list", component: ReceiverList },
  { path: "/settings", component: Settings },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
