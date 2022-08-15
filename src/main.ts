import { createApp } from 'vue'
import Antd from 'ant-design-vue'

import App from './App.vue'
import 'ant-design-vue/dist/antd.css'

import '@/assets/scss/global.scss' // 存放 margin padding flex 等快捷 class
import '@/assets/scss/common.scss' // 公共样式

createApp(App).use(Antd).mount('#app')
