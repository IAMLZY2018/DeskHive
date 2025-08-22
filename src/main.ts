import { createApp } from 'vue'
import RouterApp from './RouterApp.vue'
import router from './router'

const app = createApp(RouterApp)
app.use(router)
app.mount('#app')
