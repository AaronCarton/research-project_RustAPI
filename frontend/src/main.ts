import { App as VueApp, createApp } from 'vue'

import 'vue-toastification/dist/index.css'
import '@unocss/reset/tailwind.css'
import 'uno.css'
import './main.css'

import App from './App.vue'

import router from './bootstrap/router'
import useAuthentication from './composables/useAuthentication'

import Toast, { PluginOptions } from 'vue-toastification'
import { autoAnimatePlugin } from '@formkit/auto-animate/vue'

const app: VueApp = createApp(App)

const { restoreUser } = useAuthentication()
// const { loadUser } = useUser()

;(async function () {
  console.log('Restoring user')
  await restoreUser().then(async (fUser) => {
    console.log('Firebase token', await fUser?.value?.getIdToken())
    // if (fUser)
    //   await loadUser().then(async (user) => {
    //     console.log('Firebase token', await fUser?.value?.getIdToken())
    //     console.log('User', user.value)
    //   })
  })

  app.use(router)
  app.use(autoAnimatePlugin)
  app.use(Toast, { hideProgressBar: true, timeout: 3500 } as PluginOptions)
  app.mount('#app')
})()
