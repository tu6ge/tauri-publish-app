<script setup>
import { invoke } from '@tauri-apps/api/tauri'

const collapsed = ref(false)
const selectedKeys = ref(['oss'])
const openKeys = ref([])
const preOpenKeys = ref([])

const isOss = computed(()=>{
  return selectedKeys.value.indexOf('oss') != -1
})

const appList = ref([])

onMounted(()=>{
  get_all_app()
})

function get_all_app(){
  invoke('get_all_app').then((res)=>{
    appList.value = res.list
  })
}

const configData = ref({
  key_id: '',
  key_secret: '',
  endpoint: '',
  bucket: '',
  save_path: '',
  version_file: '',
})

import { message } from 'ant-design-vue'

function saveOssConfig(data) {
  // 保存 阿里云 的配置
  invoke('save_oss_config', {config: data}).then(res=>{
    message.success("保存成功");
  }).catch(err=>{
    console.error(err)
  })
}

onMounted(()=>{
  invoke('get_oss_config').then((res)=>{
    configData.value = ref(res).value
  }).catch(err=>{
    
  })
})

const addData = reactive({
  name: 'abc',
  path: '',
})
</script>
<template>
  
  <div class="setting">
    <div class="header">
      <h1>设置</h1>
      <router-link :to="{name:'home'}">
        <a-button >返回</a-button>
      </router-link>
    </div>
    
    <div class="setting-main">
      <a-menu
        v-model:openKeys="openKeys"
        v-model:selectedKeys="selectedKeys"
        mode="inline"
        :inline-collapsed="collapsed"
      >
        <a-menu-item key="oss">
          <span>OSS</span>
        </a-menu-item>
        <a-sub-menu key="app">
          <template #title>App</template>
          <a-menu-item v-for="(item,index) in appList" :key="index">
            <span>{{item.name}}</span>
          </a-menu-item>
        </a-sub-menu>

        
      </a-menu>

      <div class="main-content">
        <div  v-show="isOss">
          <FormKit type="form" v-model="configData" id="setting" @submit="saveOssConfig">
            
            <FormKit type="text" label="KeyId" name="key_id" validation="required"></FormKit>
            <FormKit type="text" label="KeySecret" name="key_secret" validation="required"></FormKit>
            <FormKit type="text" label="EndPoint" name="endpoint" validation="required"></FormKit>
            <FormKit type="text" label="Bucket" name="bucket" validation="required"></FormKit>
            
            <FormKit type="text" label="安装包存放目录" name="save_path" validation="required" help="设置一个 OSS 的目录，用于存放所有版本的安装包"></FormKit>
            <FormKit type="text" label="版本校验文件存储路径" name="version_file" validation="required" help="谨慎修改，修改后可能导致之前的 App 无法升级"></FormKit>
            <div class="btn-wrapper between">
              <a-button type="primary" @click="$formkit.submit('setting')">保存</a-button>
            </div>
          </FormKit>
        </div>
        <div v-show="!isOss">
          <FormKit type="form" v-model="addData" id="app_setting" @submit="saveApp">
            <FormKit type="text" label="APP 名称" name="name" readonly>
              <template #inner>
                {{addData.name}}
              </template>
            </FormKit>
            <FormKit type="text" label="安装包所在目录" name="path" readonly validation="required" placeholder="安装包所在目录">
              <template #suffix>
                <a-button type="primary" @click="selectAppPath">浏览</a-button>
              </template>
            </FormKit>
            <div class="btn-wrapper between">
              <a-button type="primary" @click="$formkit.submit('app_setting')">保存</a-button>
            </div>
          </FormKit>
        </div>
      </div>
    </div>
    
  </div>
</template>