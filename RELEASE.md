# 局域网传输工具 - 发布说明

## 🚀 快速开始

### 当前可用版本
- **macOS**: `局域网传输工具_0.1.0_aarch64.dmg` (Apple Silicon)
- **Windows**: 即将支持 - 需要在Windows环境构建
- **Linux**: 即将支持 - 需要在Linux环境构建

## 📦 多平台构建方法

### 方法一：使用GitHub Actions（推荐）

1. **推送代码到GitHub仓库**
2. **创建版本标签**：
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```
3. **自动构建**：GitHub Actions会自动构建所有平台版本
4. **下载**：在仓库的Releases页面下载对应平台的安装包

### 方法二：本地构建

#### macOS 构建
```bash
# 当前平台 (Apple Silicon)
npm run tauri build

# Intel Mac 构建
npm run tauri build -- --target x86_64-apple-darwin

# 使用构建脚本
./build-local.sh
```

#### Windows 构建
在Windows系统上执行：
```cmd
# 安装依赖
npm install

# 构建Windows版本
npm run tauri build
```

#### Linux 构建
在Linux系统上执行：
```bash
# 安装系统依赖
sudo apt-get update
sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf

# 安装项目依赖
npm install

# 构建Linux版本
npm run tauri build
```

## 📱 支持的安装包格式

### macOS
- `.app` - 应用程序包
- `.dmg` - 磁盘镜像（推荐分发格式）

### Windows
- `.msi` - Windows安装程序
- `.exe` - 单文件可执行程序

### Linux
- `.deb` - Debian/Ubuntu包
- `.rpm` - RedHat/CentOS包
- `.AppImage` - 便携式应用

## 🔧 构建要求

### 系统要求
- **Node.js**: 16.0+
- **Rust**: 1.70+
- **npm**: 8.0+

### 平台特定要求

#### macOS
- macOS 10.15+
- Xcode Command Line Tools

#### Windows
- Windows 10+
- Visual Studio Build Tools 2019+
- MSVC toolchain

#### Linux
- Ubuntu 20.04+ / 等价Linux发行版
- WebKit2GTK开发包
- 构建工具链

## 🎯 部署建议

1. **自动化构建**：使用GitHub Actions进行多平台自动构建
2. **版本管理**：使用语义化版本号（如v1.0.0）
3. **测试分发**：在目标平台上测试安装包
4. **代码签名**：为生产环境添加代码签名
5. **更新机制**：考虑集成Tauri更新插件

## 📋 发布检查清单

- [ ] 代码提交并推送
- [ ] 创建版本标签
- [ ] GitHub Actions构建成功
- [ ] 各平台安装包测试
- [ ] 发布说明编写
- [ ] Release页面发布

## 🐛 故障排除

### 构建失败
1. 检查Rust和Node.js版本
2. 清理依赖：`rm -rf node_modules package-lock.json && npm install`
3. 清理Rust缓存：`cargo clean`

### 权限问题
1. macOS: 允许未知开发者的应用
2. Windows: 添加代码签名证书
3. Linux: 检查文件权限

## 📞 技术支持

如遇到构建或使用问题，请查看：
1. 项目README文档
2. GitHub Issues页面
3. Tauri官方文档 