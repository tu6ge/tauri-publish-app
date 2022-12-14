<script setup>
import { SettingOutlined, PlusOutlined } from '@ant-design/icons-vue'
import { invoke } from '@tauri-apps/api/tauri'
import { readDir, readTextFile } from '@tauri-apps/api/fs';
import { groupBy, map, filter, every } from 'lodash-es'
import dayjs from 'dayjs'
import utc from 'dayjs/plugin/utc'
import {reset} from '@formkit/vue'
dayjs.extend(utc)

const collapsed = ref(false)
const selectedKeys = ref(['1'])
const openKeys = ref([])
const preOpenKeys = ref([])
const addAppModel = ref(false)
const initOss = ref(false)

const toggleCollapsed = () => {
  state.collapsed = !state.collapsed;
  state.openKeys = state.collapsed ? [] : state.preOpenKeys;
}

const columns = [
  { title: '名称', dataIndex: 'name', key: 'name', fixed: true },
  { title: '版本', dataIndex: 'version', key: 'version' },
  { title: '状态', dataIndex: 'status', key: 'status',
    customRender: ({value})=>{
      return value?'已上传':'未全部上传'
    }
  },
  { title: '操作', key: 'action' },
];

const configData = reactive({
  key_id: '',
  key_secret: '',
  endpoint: '',
  bucket: '',
})

import { message } from 'ant-design-vue'

function saveOssConfig(data) {
  // 保存 阿里云 的配置
  invoke('save_oss_config', {config: data}).then(res=>{
    message.success("保存成功")
    initOss.value = false
  }).catch(err=>{
    console.error(err)
  })
}

onMounted(()=>{
  invoke('get_oss_config').then((res)=>{
    configData.value = ref(res).value
  }).catch(err=>{
    initOss.value = true
  })
})
function getOssConfig(){
  
}

const appList = ref([])
const appIndex = ref(0)
const pathList = ref([])

const currentApp = computed(()=>{
  return appList.value[appIndex.value] || {}
})

watch(currentApp, async (res)=>{
  if(Object.keys(res).length==0){
    return
  }

  pathList.value = await readDir(res.path, {recursive: true})

  const checkList = await invoke('app_check_oss', {index:appIndex.value})

  pathList.value = map(pathList.value, res=>{
    res.status = checkList[res.name]
    return res
  })
})

const pathGroup = computed(()=>{
  return map(groupBy(pathList.value, re=>{
    return re.name.substr(0,re.name.indexOf('.msi'))
  }), (item,key)=>{
    const name_info = key.split('_')
    return {
      name: key,
      version: name_info[1],
      file_list: item,
      status: every(item, ['status', true]),
    }
  })
})

function uploadFiles(list){
  const files = map(list, res=>{
    return res.name
  })
  invoke('upload_files', {files, appIndex:appIndex.value}).then((re)=>{
    message.success("上传成功")
  }).catch(err=>{
    message.error("上传失败" + err)
  })
}

onMounted(()=>{
  get_all_app()
})

function get_all_app(){
  invoke('get_all_app').then((res)=>{
    appList.value = res.list
  })
}

const appData = reactive({
  name: '',
  path: '',
  oss_path:'',
  version_file: '',
})

import { open } from '@tauri-apps/api/dialog'
function selectAppPath() {
  open({
    directory: true,
    //defaultPath: await appDir(),
  }).then((path)=>{
    appData.path = path
  })
}

function saveApp(data){
  invoke('push_app', {app:data}).then((res)=>{
    get_all_app()
    reset('app_setting')
    addAppModel.value = false
  }).catch(err=>{
    console.error(err)
  })
}

const publishOpen = ref(false)
const publishInfo = reactive({
  notes: '',
  pub_date: '',
  version: '',
  signature: '',
  files: [],
  app_index: appIndex.value,
})

// TODO 更改版本号时，表单不更改
async function publish(list, version){
  publishInfo.files = map(list, res=>res.name)

  const sig_file = filter(list, res=>{
    if(res.name.substr(-3) =='sig'){
      return true
    }
    return false
  })
  if(sig_file.length==0){
    message.error("找不到签名文件")
    return
  }
  const file = sig_file[0]

  publishInfo.signature = await readTextFile(file.path)
  publishInfo.version = version

  publishOpen.value = true
}

function savePublishInfo(info){
  const pub_date = dayjs(info.pub_date, "YYYY-MM-DD\THH:mm")
  const utc = pub_date.utc().format()
  invoke('publish', {info:{...info,pub_date:utc}}).then(res=>{
    message.success("发行版本成功")
    publishOpen.value = false
  }).catch(err=>{
    message.error(err)
    console.error(err)
  })
}

import { open as openUrl } from '@tauri-apps/api/shell'
async function openVersionInfo(){

  const bucket = configData.value.bucket
  const domain = configData.value.endpoint.replace('https://', 'https://' + bucket + '.')
  await openUrl(domain + "/" + currentApp.value.version_file)
}
</script>
<template>
  <div class="flex home">
    <div class="slider">
      <a-menu
        v-model:openKeys="openKeys"
        v-model:selectedKeys="selectedKeys"
        mode="inline"
        :inline-collapsed="collapsed"
      >
        <a-menu-item v-for="(item,index) in appList" :key="index" @click="appIndex = index">
          <span>{{item.name}}</span>
        </a-menu-item>
      </a-menu>
      <a-button type="primary" @click="addAppModel=true">
        <template #icon>
          <PlusOutlined />
        </template>
        添加 APP
      </a-button>
    </div>
    <a-layout>
      <a-layout-header>
        <!-- <a-button type="primary" @click="toggleCollapsed">
          <MenuUnfoldOutlined v-if="collapsed" />
          <MenuFoldOutlined v-else />
        </a-button> -->
        <h1>安装包列表</h1>
        <router-link :to="{name:'setting'}">
          <a-button shape="circle">
            <template #icon><SettingOutlined /></template>
          </a-button>
        </router-link>
        
      </a-layout-header>
      <a-layout-content>
        <a-table :columns="columns" :data-source="pathGroup">
          <template #bodyCell="{ column, record }">
            <template v-if="column.key === 'action'">
              <a @click="uploadFiles(record.file_list)">上传</a>
              <a class="ml20" @click="publish(record.file_list, record.version)">上传并发布</a>
            </template>
          </template>
          <template #expandedRowRender="{ record }">
            <p style="margin: 0" v-for="(file, k) in record.file_list" :key="k">
              {{ file.name }} <a>{{file.status?'已上传':'未上传'}}</a>
            </p>
          </template>
        </a-table>
      </a-layout-content>
      <!-- <a-layout-footer>Footer</a-layout-footer> -->
    </a-layout>

    <a-modal v-model:visible="addAppModel" title="新增 App" @ok="$formkit.submit('app_setting')">
      <FormKit type="form" v-model="appData" id="app_setting" @submit="saveApp">
        <FormKit type="text" label="安装包所在目录" name="path" readonly validation="required" placeholder="安装包所在目录"
          help="选择 xxx\src-tauri\target\release\bundle\msi 目录，目前只支持 msi"
        >
          <template #suffix>
            <a-button type="primary" @click="selectAppPath">浏览</a-button>
          </template>
        </FormKit>
        <FormKit type="text" label="APP 名称" name="name" validation="required"></FormKit>
        <FormKit type="text" label="安装包存放目录" name="oss_path" validation="required" help="设置一个 OSS 的目录，用于存放所有版本的安装包"></FormKit>
        <FormKit type="text" label="版本校验文件存储路径" name="version_file" validation="required" help="谨慎修改，修改后可能导致之前的 App 无法升级"></FormKit>
      </FormKit>
    </a-modal>
    <a-modal v-model:visible="initOss" title="初始化 OSS 配置" @ok="$formkit.submit('home_setting')" :cancelText="false"
      okText="保存"
      wrapClassName="home-setting"
     :maskClosable="false" :closable="false">
      <FormKit type="form" submit-label="保存" v-model="configData" id="home_setting" @submit="saveOssConfig">
        <FormKit type="text" label="KeyId" name="key_id" validation="required"></FormKit>
        <FormKit type="text" label="KeySecret" name="key_secret" validation="required"></FormKit>
        <FormKit type="text" label="EndPoint" name="endpoint" validation="required" placeholder="https://oss-cn-shanghai.aliyuncs.com"></FormKit>
        <FormKit type="text" label="Bucket" name="bucket" validation="required" ></FormKit>
      </FormKit>
    </a-modal>
    <a-modal v-model:visible="publishOpen" title="发布新版本" @ok="$formkit.submit('publish')">
      <FormKit type="form" v-model="publishInfo" id="publish" @submit="savePublishInfo" submit-label="发布">
        <FormKit type="textarea" label="版本说明" name="notes" validation="required"></FormKit>
        <FormKit type="datetime-local" label="发行时间" name="pub_date" validation="required"></FormKit>
        <FormKit type="text" label="版本号" name="version" placeholder="请输入要发布的版本号" validation="required"></FormKit>
        <FormKit type="textarea" label="签名信息" name="signature" validation="required" ></FormKit>
        <a @click="openVersionInfo">查看当前版本信息</a>
      </FormKit>
    </a-modal>
    
  </div>
</template>
