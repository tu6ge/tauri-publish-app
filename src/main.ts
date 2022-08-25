import { createApp } from 'vue'
import Antd from 'ant-design-vue'
import { createPinia } from 'pinia'

import App from './App.vue'
// import 'ant-design-vue/dist/antd.css'
import router from './router'

import { zh } from '@formkit/i18n'
import { plugin, defaultConfig } from '@formkit/vue'
import 'ant-design-vue/es/message/style/css'

import '@formkit/themes/genesis'
import '@/assets/scss/global.scss' // 存放 margin padding flex 等快捷 class
import '@/assets/scss/common.scss' // 公共样式

createApp(App)
  .use(router)
  .use(Antd)
  .use(createPinia())
  .use(plugin, defaultConfig({
    locales: { zh },
    locale: 'zh'
  }))
  .mount('#app')
