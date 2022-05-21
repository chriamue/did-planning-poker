import { createApp } from "vue";
import App from "./App.vue";
import { createPinia } from "pinia";
import BootstrapVue3 from "bootstrap-vue-3";
import "bootstrap/dist/css/bootstrap.css";
import "bootstrap-vue-3/dist/bootstrap-vue-3.css";

import init, { init_panic_hook } from "did_planning_poker";
init().then(() => {
  init_panic_hook();
  console.log("wasm loaded");
});

const app = createApp(App);
app.use(createPinia());
app.use(BootstrapVue3);
app.mount("#app");
