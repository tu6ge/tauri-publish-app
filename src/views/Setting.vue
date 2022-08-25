<script setup>
import { invoke } from '@tauri-apps/api/tauri'

// import {useOssStore} from '@/store/setting.js'

// const ossStore = useOssStore()

const collapsed = ref(false)
const selectedKeys = ref(['oss'])
const openKeys = ref([])
const preOpenKeys = ref([])

const isOss = computed(()=>{
  return selectedKeys.value.indexOf('oss') != -1
})

const appList = ref([])
const appIndex = ref(0)
onMounted(()=>{
  invoke('get_oss_config').then((res)=>{
    configData.value = ref(res).value
  }).catch(err=>{
    
  })

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



const appData = ref({
  name: '',
  path: '',
  oss_path:'',
  version_file: '',
})

function selectApp(index){
  appIndex.value = index

  appData.value = appList.value[index]
}

import { open } from '@tauri-apps/api/dialog'
function selectAppPath() {
  open({
    directory: true,
    //defaultPath: await appDir(),
  }).then((path)=>{
    appData.value.path = path
  })
}

function saveApp(app){
  invoke('update_app', {index: appIndex.value, app}).then((res)=>{
    appList.value[appIndex.value] = app
    message.success("保存成功")
  }).catch(err=>{
    console.error(err)
  })
}

async function removeApp(){
  await invoke('remove_app', {index: appIndex.value})
  appList.value.splice(appIndex.value, 1)
  selectedKeys.value = [0]
  selectApp(0)
  message.success("已移除")
}
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
          <a-menu-item v-for="(item,index) in appList" :key="index" @click="selectApp(index)">
            <span>{{item.name}}</span>
          </a-menu-item>
        </a-sub-menu>

        
      </a-menu>

      <div class="main-content">
        <div  v-show="isOss">
          <FormKit type="form" v-model="configData" id="setting" @submit="saveOssConfig">
            
            <FormKit type="text" label="KeyId" name="key_id" validation="required"></FormKit>
            <FormKit type="text" label="KeySecret" name="key_secret" validation="required"></FormKit>
            <FormKit type="text" label="EndPoint" name="endpoint" validation="required"
              help="tauri 要求必须用 https"
            placeholder="https://oss-cn-shanghai.aliyuncs.com"></FormKit>
            <FormKit type="text" label="Bucket" name="bucket" validation="required"></FormKit>
            
            <div class="btn-wrapper between">
              <a-button type="primary" @click="$formkit.submit('setting')">保存</a-button>
            </div>
          </FormKit>
        </div>
        <div v-show="!isOss">
          <FormKit type="form" v-model.sync="appData" id="app_setting" @submit="saveApp">
            <FormKit type="text" label="APP 名称" name="name">
              <template #inner>
                {{appData.name}}
              </template>
            </FormKit>
            <FormKit type="text" label="安装包所在目录" name="path" readonly validation="required" placeholder="安装包所在目录">
              <template #suffix>
                <a-button type="primary" @click="selectAppPath">浏览</a-button>
              </template>
            </FormKit>
            <FormKit type="text" label="安装包存放目录" name="oss_path"
              placeholder="apps"
            validation="required" help="设置一个 OSS 的目录，用于存放所有版本的安装包"></FormKit>
            <FormKit type="text" label="版本校验文件存储路径" name="version_file" validation="required" help="谨慎修改，修改后可能导致之前的 App 无法升级"></FormKit>
            <div class="btn-wrapper between">
              <a-button type="primary" @click="$formkit.submit('app_setting')">保存</a-button>
              <a-popconfirm
                title="确定移除该 App 吗"
                ok-text="移除"
                cancel-text="取消"
                @confirm="removeApp"
              >
                <a-button type="default" >移除App</a-button>
              </a-popconfirm>
            </div>
          </FormKit>
        </div>
      </div>
    </div>
    
  </div>
</template>