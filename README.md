# Photo Organizer

一个功能强大的桌面照片管理工具，支持从多种来源导入照片，自动检测重复照片，并按自定义规则分类整理。

## 功能特性

- **多源导入**：支持从 SD 卡、安卓手机、iOS 设备和本地文件夹导入照片
- **智能分类**：按主题、日期、设备类型、文件类型自动分类整理
- **重复检测**：基于文件哈希值检测重复照片
- **预览功能**：缩略图网格和大图预览
- **批量导出**：支持自定义导出路径和文件夹结构
- **安全记录**：使用 SQLite 记录导入历史，避免重复导入

## 技术栈

### 前端
- **Vue 3** - 渐进式 JavaScript 框架
- **TypeScript** - 类型安全的 JavaScript 超集
- **Pinia** - Vue 3 状态管理库
- **TailwindCSS** - 原子化 CSS 框架
- **Vite** - 下一代前端构建工具
- **Lucide Icons** - 美观的图标库

### 后端
- **Tauri** - 轻量级桌面应用框架
- **Rust** - 系统级编程语言
- **SQLite** - 嵌入式数据库
- **Rusqlite** - Rust 的 SQLite 绑定库
- **SHA2** - 密码哈希算法库

## 安装前置条件

在运行项目之前，请确保已安装以下工具：

1. **Node.js** (推荐 v18 或更高版本)
   - 下载地址：https://nodejs.org/

2. **Rust** 和 **Cargo**
   - 安装命令：
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   - 或访问：https://www.rust-lang.org/tools/install

3. **系统构建工具**
   - **Windows**：Visual Studio C++ Build Tools
   - **macOS**：Xcode Command Line Tools
   - **Linux**：gcc 和 make

## 项目结构

```
photo-organizer/
├── src/
│   ├── components/         # Vue 组件
│   │   ├── PhotoGrid.vue  # 照片网格组件
│   │   ├── SourcePanel.vue# 来源面板组件
│   │   ├── StatusBar.vue  # 状态栏组件
│   │   └── Toolbar.vue    # 工具栏组件
│   ├── stores/            # Pinia 状态管理
│   │   └── photoStore.ts  # 照片状态存储
│   ├── types/             # TypeScript 类型定义
│   │   └── index.ts       # 核心类型
│   ├── App.vue            # 主应用组件
│   ├── main.ts            # 前端入口
│   ├── style.css          # 全局样式
│   ├── main.rs            # Tauri 主程序入口
│   ├── db.rs              # 数据库操作
│   ├── file_ops.rs        # 文件操作
│   ├── hash.rs            # 哈希计算
│   └── lib.rs             # Rust 库模块
├── dist/                  # 构建输出目录
├── docs/                  # 文档目录
│   └── superpowers/
│       ├── plans/         # 项目计划
│       └── specs/         # 设计文档
├── Cargo.toml             # Rust 项目配置
├── package.json           # 前端项目配置
├── tauri.conf.json        # Tauri 配置
├── vite.config.ts         # Vite 配置
├── tsconfig.json          # TypeScript 配置
└── index.html             # HTML 入口
```

## 安装和运行

### 1. 克隆项目

```bash
git clone <repository-url>
cd photo-organizer
```

### 2. 安装依赖

```bash
npm install
```

### 3. 开发模式运行

```bash
npm run tauri dev
```

这将同时启动：
- Vite 开发服务器（前端）
- Tauri 应用窗口（后端）

### 4. 构建生产版本

```bash
npm run tauri build
```

构建产物将生成在：
- **Windows**：`src-tauri/target/release/bundle/msi/`
- **macOS**：`src-tauri/target/release/bundle/macos/`
- **Linux**：`src-tauri/target/release/bundle/appimage/`

## 其他可用命令

### 仅启动前端开发服务器

```bash
npm run dev
```

### 仅构建前端

```bash
npm run build
```

### 预览前端构建结果

```bash
npm run preview
```

## 使用说明

### 导入照片

1. 在左侧面板选择导入来源（设备或本地文件夹）
2. 等待扫描完成
3. 在右侧网格中查看待处理照片

### 配置分类规则

1. 在左侧面板的「分类规则」区域设置：
   - 主题名称
   - 是否按设备类型分类
   - 是否按文件类型分类

### 选择和导出

1. 点击照片缩略图进行选择/取消选择
2. 使用工具栏按钮全选/取消选择
3. 点击「显示重复」查看重复照片
4. 点击「导出」选择目标位置并开始导出

### 文件夹结构示例

导出后，照片将按照以下结构组织：

```
photo/
└── 2025年/
    └── 20250529_千岛湖湖心谷/
        ├── 360/
        ├── 富士视频/
        └── 富士照片/
```

## 开发文档

- **设计文档**：[docs/superpowers/specs/2026-05-19-photo-organizer-design.md](file:///workspace/docs/superpowers/specs/2026-05-19-photo-organizer-design.md)
- **项目计划**：[docs/superpowers/plans/2026-05-19-photo-organizer-plan.md](file:///workspace/docs/superpowers/plans/2026-05-19-photo-organizer-plan.md)

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT License
