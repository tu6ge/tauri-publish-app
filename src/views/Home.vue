
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
        <FormKit type="text"></FormKit>
        <FormKit type="text"></FormKit>
        <FormKit type="text"></FormKit>
      </FormKit>
    </a-modal>
  </div>
</template>
<script>
export default {
  setup() {
    const state = reactive({
      collapsed: false,
      selectedKeys: ['1'],
      openKeys: ['sub1'],
      preOpenKeys: ['sub1'],
      addAppModel: false,
    })

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

    return {
      ...toRefs(state),
      toggleCollapsed,
      columns,
      data
    }

  },
  data(){
    return {
      list:[],
    }
  }
}
</script>