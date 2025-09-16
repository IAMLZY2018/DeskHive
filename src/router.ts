import { createRouter, createWebHistory } from 'vue-router'
import App from './App.vue'
import Settings from './Settings.vue'
import Calendar from './Calendar.vue'

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
  {
    path: '/calendar',
    name: 'Calendar',
    component: Calendar
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router