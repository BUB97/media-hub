import { createApp } from 'vue';
import './style.css';
import App from './App.vue';
import router from './router';
import { initApp } from './utils/getApp';

const app = createApp(App);
app.use(router);

// 初始化全局应用管理器
initApp(router);

app.mount('#app');
