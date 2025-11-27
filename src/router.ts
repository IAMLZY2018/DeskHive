import { createRouter, createWebHashHistory } from 'vue-router'
import App from './App.vue'
import Settings from './Settings.vue'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: App
  },
  {
    path: '/settings',
    name: 'Settings', 
    component: Settings
  },

]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router