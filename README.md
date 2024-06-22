# Rust-Reader

Rust-Reader 是一个简单、轻量、美观的 Epub 阅读器，由 Rust、Vue 和 TypeScript 编写，使用 Tauri 框架构建。

该项目还在快速迭代中，欢迎大家参与讨论和贡献。更详细的文档将在稍后发布。

## 更新内容

1. 优化了程序界面的布局和样式；
2. 新增了一些组件与动画；
3. 修复了一些 bug，增加了程序的健壮性；

## 运行环境

该项目主要在 Windows11 下开发与测试，尚未测试过其他操作系统或 Rust。

以下是开发时环境：

-   Rust 1.77.1
-   Windows 11

## 快速开始

1. 安装 Rust 和 Node.js
2. 安装依赖
    ```shell
    cargo install create-tauri-app --locked
    cargo install tauri-cli
    npm install # 需要切换到项目目录下
    ```
3. 运行：
    ```shell
    cargo tauri dev # 需要切换到项目目录下
    ```
4. 构建安装包：
    ```shell
    cargo tauri build # 需要切换到项目目录下
    ```

## 一些 Q & A

-   **Why Tauri?**：没有什么特殊原因，学了一点 Rust，浅试一下吧~
-   **Why Vue?**：也没有什么特殊原因，周围人都在学，浅试一下吧~
-   **Why Chinese?**：因为中文是我的母语，懒得翻译了...

## 项目结构

本部分展示了该项目下部分文件和目录的用处。

```shell
│  .gitignore                           # Git 忽略文件
│  .prettierrc.json                     # Prettier 配置文件
│  index.html
│  LICENSE
│  package-lock.json                    # NPM 配置文件
│  package.json                         # NPM 配置文件
│  README.md
│  tsconfig.json                        # https://code.visualstudio.com/docs/languages/jsconfig
│  tsconfig.node.json
│  vite.config.ts                       # Vite 配置文件
├─.vscode                               # VSCode 配置目录
├─public
│      config.yml                       # 本软件的配置模板
├─src                                   # 前端目录
│  │  App.vue
│  │  main.ts
│  │  vite-env.d.ts
│  ├─assets
│  │  ├─css                             # 全局样式
│  │  └─fonts                           # 字体文件
│  ├─components                         # 各类组件
│  ├─core                               # 核心模块
│  ├─entity                             # 实体类
│  ├─router                             # Router 配置
│  ├─store
│  │      appStateStore.ts              # 程序状态 Store
│  │      index.ts
│  │      settingStore.ts               # 程序设置 Store
│  ├─utils                              # 工具类
│  └─views
│      │  MainPanel.vue
│      ├─page                           # 主页面
│      └─sidebar                        # 侧边栏页面
│
└─src-tauri                             # 后端目录
    │  .gitignore                       # 子项目的 Git 忽略文件
    │  build.rs                         # Rust 项目构建脚本
    │  Cargo.lock
    │  Cargo.toml                       # Rust 项目配置文件
    │  tauri.conf.json                  # Tauri 配置文件
    ├─icons                             # Tauri 图标目录
    └─src
       │  lib.rs
       │  main.rs
       ├─entity                         # Rust 实体类
       ├─handler
       │      book_handler.rs           # 书籍相关 handler
       │      book_list_handler.rs      # 书籍列表相关 handler
       │      config_handler.rs         # 配置相关 handler
       │      mod.rs
       │      read_handler.rs           # 阅读相关 handler
       └─utils
```

## 免责申明

本软件遵循 MIT 协议，您可以自由使用、修改和分发，请遵守你所在地区的本地法律，原作者不对使用本软件造成的任何后果负责。
