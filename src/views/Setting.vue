<script setup>
</script>
<template>
  <div class="setting">
    <FormKit type="form">
      <h1>设置</h1>
      <FormKit type="text" label="KeyId" validation="required"></FormKit>
      <FormKit type="text" label="KeySecret" validation="required"></FormKit>
      <FormKit type="text" label="EndPoint" validation="required"></FormKit>
      <FormKit type="text" label="Bucket" validation="required"></FormKit>
     
      <FormKit type="text" label="安装包存放目录" validation="required" help="设置一个目录，用于存放所有版本的安装包"></FormKit>
      <FormKit type="text" label="版本校验文件存储路径" validation="required" help="谨慎修改，修改后可能导致之前的 App 无法升级"></FormKit>
      <div class="btn-wrapper between">
        <a-button type="primary">保存</a-button>
        <router-link :to="{name:'home'}">
          <a-button >取消</a-button>
        </router-link>
        
      </div>
    </FormKit>
    
  </div>
</template>