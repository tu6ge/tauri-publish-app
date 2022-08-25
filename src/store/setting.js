import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/tauri'

export const useOssStore = defineStore('oss', {
  state: () => {
    return {
      config: null,
    }
  },
  actions: {
    async getOssConfig() {
      this.config = await invoke('get_oss_config')
    },

    async updateOssConfig(config) {
      await invoke('save_oss_config', {config})
      this.config = config
    }
  }
})

export const useAppStore = defineStore('app', {
  state: () => {
    return {
      index: 0,
      list: [],
    }
  },
  actions: {
    getAppList(){
      invoke('get_all_app').then((res)=>{
        this.list = res.list
      })
    }
  }
})