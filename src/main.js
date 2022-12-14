import { createApp } from "vue";
import "./style.css";
import "bootstrap/dist/css/bootstrap.min.css";
import "bootstrap/dist/js/bootstrap.bundle";
import "@fortawesome/fontawesome-free/css/all.min.css";
import App from "./App.vue";
import mitt from 'mitt';

const emitter = mitt();
const app = createApp(App);

app.config.globalProperties.emitter = emitter;
app.mount("#app");
