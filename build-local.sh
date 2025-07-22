#!/bin/bash

# 局域网传输工具 - 多平台构建脚本
# 支持 macOS、Windows、Linux 平台构建

set -e

echo "🚀 开始构建局域网传输工具..."

# 清理之前的构建
echo "🧹 清理之前的构建文件..."
rm -rf src-tauri/target/release/bundle/
npm run build

echo "📦 开始构建生产版本..."

# 构建当前平台
npm run tauri build

echo ""
echo "✅ 构建完成！生成的文件位置："
echo ""

# 检查生成的文件
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "🍎 macOS 版本："
    find src-tauri/target/release/bundle/ -name "*.app" -o -name "*.dmg" | while read file; do
        echo "  📱 $(basename "$file")"
        echo "     $(realpath "$file")"
    done
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "🐧 Linux 版本："
    find src-tauri/target/release/bundle/ -name "*.deb" -o -name "*.rpm" -o -name "*.AppImage" | while read file; do
        echo "  📦 $(basename "$file")"
        echo "     $(realpath "$file")"
    done
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    echo "🪟 Windows 版本："
    find src-tauri/target/release/bundle/ -name "*.msi" -o -name "*.exe" | while read file; do
        echo "  💿 $(basename "$file")"
        echo "     $(realpath "$file")"
    done
fi

echo ""
echo "🎉 构建完成！"

# 提供使用说明
echo ""
echo "📋 使用说明："
echo "1. 在每台设备上安装对应版本的应用"
echo "2. 确保设备连接在同一局域网内"
echo "3. 启动应用后点击'扫描设备'"
echo "4. 选择文件并发送到目标设备"
echo ""
echo "📂 所有安装包位于: src-tauri/target/release/bundle/" 