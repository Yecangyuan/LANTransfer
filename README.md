# 局域网传输工具

一个基于 Tauri 开发的跨平台局域网文件传输工具，支持 Windows、macOS 和 Linux。

## 功能特性

- 🚀 **快速传输**: 基于 TCP 的高速文件传输
- 🔍 **自动发现**: 使用 mDNS 自动发现局域网内的设备
- 📱 **跨平台**: 支持 Windows、macOS、Linux 桌面平台
- 🎨 **现代界面**: 使用 React + Tailwind CSS 构建的美观界面
- 🔒 **安全可靠**: 仅在局域网内传输，数据不经过外部服务器
- 📊 **实时进度**: 实时显示文件传输进度

## 技术栈

- **前端**: React + TypeScript + Tailwind CSS
- **后端**: Rust + Tauri
- **网络**: mDNS 设备发现 + TCP 文件传输
- **构建**: Vite + Tauri CLI

## 系统要求

- Node.js 16+
- Rust 1.70+
- 支持的操作系统: Windows 10+, macOS 10.15+, Linux

## 安装依赖

```bash
# 安装前端依赖
npm install

# 安装 Tauri CLI (如果还没有安装)
npm install -g @tauri-apps/cli
```

## 开发运行

```bash
# 启动开发服务器
npm run tauri:dev
```

## 构建

```bash
# 构建生产版本
npm run tauri:build
```

构建完成后，可执行文件位于 `src-tauri/target/release/bundle/` 目录下。

## 使用说明

1. **启动应用**: 运行应用程序
2. **扫描设备**: 点击"扫描设备"按钮发现局域网内的其他设备
3. **选择文件**: 点击"选择文件"按钮选择要传输的文件
4. **发送文件**: 选择目标设备后点击"发送文件"
5. **查看进度**: 在传输进度区域查看文件传输状态

## 网络端口

- **mDNS 服务发现**: 使用系统默认 mDNS 端口
- **文件传输**: 8081 端口 (TCP)
- **服务注册**: 8080 端口

## 故障排除

### 无法发现设备
- 确保设备在同一局域网内
- 检查防火墙设置，允许应用访问网络
- 确保 mDNS 服务正常运行

### 文件传输失败
- 检查网络连接
- 确保目标设备上的应用正在运行
- 检查磁盘空间是否足够

## 开发说明

### 项目结构

```
LANTransfer/
├── src/                    # 前端源码
│   ├── App.tsx            # 主应用组件
│   ├── main.tsx           # 应用入口
│   └── styles.css         # 样式文件
├── src-tauri/             # Rust 后端源码
│   ├── src/
│   │   ├── main.rs        # 主程序入口
│   │   ├── device.rs      # 设备管理
│   │   ├── network.rs     # 网络发现
│   │   └── file_transfer.rs # 文件传输
│   ├── Cargo.toml         # Rust 依赖配置
│   └── tauri.conf.json    # Tauri 应用配置
├── package.json           # Node.js 依赖配置
├── vite.config.ts         # Vite 构建配置
└── tailwind.config.js     # Tailwind CSS 配置
```

### 添加新功能

1. **前端功能**: 修改 `src/App.tsx` 和相关组件
2. **后端功能**: 在 `src-tauri/src/` 目录下添加新模块
3. **API 接口**: 在 `src-tauri/src/main.rs` 中添加新的 Tauri 命令

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！ 