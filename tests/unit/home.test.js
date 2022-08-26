import { test, assert, describe } from 'vitest'
import { mount } from '@vue/test-utils'
import {nextTick} from 'vue'
import Home from '../../src/views/Home.vue'
import { mockIPC } from "@tauri-apps/api/mocks"
import { expect, beforeAll } from 'vitest'
import { randomFillSync } from "crypto"

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
    }
  })
})


test('guide set OSS setting', async ()=>{

  

  const wrapper = mount(Home)

  await nextTick()
  
  expect(wrapper.vm.initOss).toBe(true)
})