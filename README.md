# Tauri Publish App

用 aliyun OSS 管理 tauri 项目版本的一个桌面应用

## 使用到的

- [tauri-vue-template](https://github.com/Uninen/tauri-vue-template)
- [aliyun-oss-client](https://github.com/tu6ge/oss)

## 功能

- 可以将本地打包的 msi 文件一键上传到 OSS
- 可以一键修改发行版本的服务器配置文件
- 借助以下配置，结合 Tauri 文档说明，可实现应用的自动更新

```json
"tauri":{
  ...
  "updater": {
    "active": true,
    "endpoints": [
        "https://xxx.aliyuncs.com/app-config.json"
    ],
    "dialog": true,
    "pubkey": "xxxxxxxxxxxxxxxxx"
  },
}

```
