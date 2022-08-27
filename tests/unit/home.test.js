import { test, assert, describe } from 'vitest'
import { mount } from '@vue/test-utils'
import {nextTick} from 'vue'
import Home from '../../src/views/Home.vue'
import { mockIPC } from "@tauri-apps/api/mocks"
import { expect, beforeAll } from 'vitest'
import { randomFillSync } from "crypto"
import {Button, Modal, Menu, Layout, Table} from 'ant-design-vue'
import {RouterLink} from 'vue-router'
import { zh } from '@formkit/i18n'
import { plugin, defaultConfig } from '@formkit/vue'

beforeAll(() => {
  //@ts-ignore
  window.crypto = {
    getRandomValues: function (buffer) {
      return randomFillSync(buffer)
    },
  }


  mockIPC((cmd, args) => {
    // simulated rust command called "add" that just adds two numbers
    if(cmd === "get_oss_config") {
      throw 'abc'
    }else if(cmd === 'push_app'){
      return ''
    }else if(cmd === 'get_all_app'){
      return {
        list: [],
        index: 0,
      }
    }
  })
})


test('guide set OSS setting', async ()=>{

  const wrapper = await mount(Home, {
    global:{
      components:{
        [Button.name]:Button,
        [Modal.name]: Modal,
        [Menu.name]:Menu,
        [Menu.Item.name]: Menu.Item,
        [Layout.name]: Layout,
        [Layout.Header.name]: Layout.Header,
        [Layout.Footer.name]: Layout.Footer,
        [Layout.Content.name]: Layout.Content,
        [Table.name]: Table,
      },
      plugins: [
        [
          plugin, 
          defaultConfig({
            locales: { zh },
            locale: 'zh'
          })
        ]
      ]
    }
  })

  assert.equal(wrapper.vm.initOss, false)
  
  setTimeout(() =>{
    assert.equal(wrapper.vm.initOss, true)
  }, 2000)
  
})