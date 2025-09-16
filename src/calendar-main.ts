import { createApp } from 'vue';
import RouterApp from './RouterApp.vue';
import router from './router';

// 创建Vue应用实例
const app = createApp(RouterApp);

// 使用路由
app.use(router);

// 挂载应用到DOM
app.mount('#app');

// 监听路由变化以更新窗口装饰
router.afterEach(async (to) => {
  const { getCurrentWindow } = await import('@tauri-apps/api/window');
  const currentWindow = getCurrentWindow();
  
  // 根据当前路由设置窗口是否需要装饰
  if (to.name === 'Calendar') {
    // 日历窗口需要装饰
    await currentWindow.setDecorations(true);
  } else {
    // 其他窗口不需要装饰
    await currentWindow.setDecorations(false);
  }
});