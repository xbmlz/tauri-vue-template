import { createApp } from 'vue'
import { setupLayouts } from 'virtual:generated-layouts'
import { routes } from 'vue-router/auto-routes'
import { createRouter, createWebHistory } from 'vue-router'
import App from './App.vue'

import '@unocss/reset/tailwind.css'
import './styles.css'
import 'uno.css'

const app = createApp(App)

const router = createRouter({
  history: createWebHistory(),
  routes: setupLayouts(routes),
})

app.use(router)
app.mount('#app')
