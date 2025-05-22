import { createApp } from "vue";
import App from "./App.vue";
import Antd from 'ant-design-vue';
import 'ant-design-vue/dist/reset.css';
import store from "./store/index";
import router from "./router";

const app = createApp(App);
app.use(store);
app.use(router)
app.use(Antd).mount("#app"); 
