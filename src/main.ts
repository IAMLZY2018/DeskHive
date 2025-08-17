import { createApp } from 'vue'
import App from './App.vue'

// 禁用 Vue DevTools
const app = createApp(App)
app.config.devtools = false
app.mount('#app')
