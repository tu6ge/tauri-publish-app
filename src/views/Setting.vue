<script setup>
import { invoke } from '@tauri-apps/api/tauri'

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
</script>
<template>
  <div class="setting">
    <FormKit type="form" v-model="configData" id="setting" @submit="saveOssConfig">
      <h1>设置</h1>
      <FormKit type="text" label="KeyId" name="key_id" validation="required"></FormKit>
      <FormKit type="text" label="KeySecret" name="key_secret" validation="required"></FormKit>
      <FormKit type="text" label="EndPoint" name="endpoint" validation="required"></FormKit>
      <FormKit type="text" label="Bucket" name="bucket" validation="required"></FormKit>
      
      <FormKit type="text" label="安装包存放目录" name="save_path" validation="required" help="设置一个 OSS 的目录，用于存放所有版本的安装包"></FormKit>
      <FormKit type="text" label="版本校验文件存储路径" name="version_file" validation="required" help="谨慎修改，修改后可能导致之前的 App 无法升级"></FormKit>
      <div class="btn-wrapper between">
        <a-button type="primary" @click="$formkit.submit('setting')">保存</a-button>
        <router-link :to="{name:'home'}">
          <a-button >返回</a-button>
        </router-link>
        
      </div>
    </FormKit>
    
  </div>
</template>