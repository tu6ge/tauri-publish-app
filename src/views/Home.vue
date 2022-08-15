<script setup>
import { open } from '@tauri-apps/api/dialog'
const collapsed = ref(false)
const selectedKeys = ref(['1'])
const openKeys = ref(['sub1'])
const preOpenKeys = ref(['sub1'])
const addAppModel = ref(false)
const initOss = ref(true)

const toggleCollapsed = () => {
  state.collapsed = !state.collapsed;
  state.openKeys = state.collapsed ? [] : state.preOpenKeys;
}

const columns = [
  { title: 'Name', dataIndex: 'name', key: 'name', fixed: true },
  { title: 'Age', dataIndex: 'age', key: 'age' },
  { title: 'Address', dataIndex: 'address', key: 'address' },
  { title: 'Action', key: 'action' },
];

const data = [
  {
    key: 1,
    name: 'John Brown',
    age: 32,
    address: 'New York No. 1 Lake Park',
    description: 'My name is John Brown, I am 32 years old, living in New York No. 1 Lake Park.',
  },
]

function saveOssConfig() {
  // 保存 阿里云 的配置
}

function selectAppPath() {
  open({
    directory: true,
    //defaultPath: await appDir(),
  }).then((path)=>{
    console.log(path)
  })
}
</script>
<template>
  <div class="flex">
    <div class="slider">
      <a-menu
        v-model:openKeys="openKeys"
        v-model:selectedKeys="selectedKeys"
        mode="inline"
        :inline-collapsed="collapsed"
      >
        <a-menu-item key="1">
          <template #icon>
            <PieChartOutlined />
          </template>
          <span>Option 1</span>
        </a-menu-item>
        <a-menu-item key="2">
          <template #icon>
            <DesktopOutlined />
          </template>
          <span>Option 2</span>
        </a-menu-item>
        <a-menu-item key="3">
          <template #icon>
            <InboxOutlined />
          </template>
          <span>Option 3</span>
        </a-menu-item>
        <a-sub-menu key="sub1">
          <template #icon>
            <MailOutlined />
          </template>
          <template #title>Navigation One</template>
          <a-menu-item key="5">Option 5</a-menu-item>
          <a-menu-item key="6">Option 6</a-menu-item>
          <a-menu-item key="7">Option 7</a-menu-item>
          <a-menu-item key="8">Option 8</a-menu-item>
        </a-sub-menu>
      </a-menu>
      <a-button type="primary" @click="addAppModel=true">添加 APP</a-button>
    </div>
    <a-layout>
      <a-layout-header>
        <a-button type="primary" @click="toggleCollapsed">
          <MenuUnfoldOutlined v-if="collapsed" />
          <MenuFoldOutlined v-else />
        </a-button>
        Header
      </a-layout-header>
      <a-layout-content>
        <a-table :columns="columns" :data-source="data">
          <template #bodyCell="{ column }">
            <template v-if="column.key === 'action'">
              <a>Delete</a>
            </template>
          </template>
          <template #expandedRowRender="{ record }">
            <p style="margin: 0">
              {{ record.description }}
            </p>
          </template>
        </a-table>
      </a-layout-content>
      <!-- <a-layout-footer>Footer</a-layout-footer> -->
    </a-layout>

    <a-modal v-model:visible="addAppModel" title="Basic Modal" @ok="handleOk">
      <FormKit type="form">
        <FormKit type="text" label="APP 包所在目录" readonly placeholder="安装包所在目录">
          <template #suffix>
            <a-button type="primary" @click="selectAppPath">浏览</a-button>
          </template>
        </FormKit>
        <FormKit type="text" label="APP 名称"></FormKit>
      </FormKit>
    </a-modal>
    <a-modal v-model:visible="initOss" title="初始化 OSS 配置" @ok="saveOssConfig">
      <FormKit type="form" submit-label="保存">
        <FormKit type="text" label="KeyId" validation="required"></FormKit>
        <FormKit type="text" label="KeySecret" validation="required"></FormKit>
        <FormKit type="text" label="EndPoint" validation="required"></FormKit>
        <FormKit type="text" label="Bucket" validation="required"></FormKit>
      </FormKit>
    </a-modal>
  </div>
</template>
