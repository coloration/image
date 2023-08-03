import { createApp } from 'vue'
import { createHead } from '@vueuse/head'
import App from './App.vue'
import { router } from './router'
import './style'

const app = createApp(App)

app.use(createHead())
app.use(router)

app.mount('#app')
