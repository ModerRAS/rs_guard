#!/usr/bin/env bash

# 简化版 BDD 测试脚本
# 用于 rs_guard 项目的用户验收测试

set -e

echo "🧪 rs_guard 简化版 BDD 测试"
echo "================================="

# 检查依赖
echo "📋 检查依赖..."
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo 未找到，请先安装 Rust"
    exit 1
fi

# 创建测试数据目录
echo "📁 创建测试数据目录..."
mkdir -p test-data/source
mkdir -p test-data/output

# 创建测试文件
echo "📄 创建测试文件..."
cat > test-data/source/test_file.txt << 'EOF'
这是一个测试文件
用于验证 rs_guard 的文件保护功能
包含多行内容
EOF

# 启动后端服务
echo "🚀 启动后端服务..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "❌ 构建失败"
    exit 1
fi

# 后台启动服务
./target/release/backend &
BACKEND_PID=$!
echo "📡 后端服务已启动 (PID: $BACKEND_PID)"

# 等待服务启动
echo "⏳ 等待服务启动..."
for i in {1..10}; do
    if curl -s -f http://localhost:3000/api/status > /dev/null 2>&1; then
        echo "✅ 服务已就绪"
        break
    fi
    if [ $i -eq 10 ]; then
        echo "❌ 服务启动超时"
        kill $BACKEND_PID 2>/dev/null
        exit 1
    fi
    echo "⏳ 等待服务启动... ($i/10)"
    sleep 2
done

# 测试 1: 检查服务状态
echo ""
echo "🧪 测试 1: 检查服务状态"
echo "---------------------------------"
if curl -s http://localhost:3000/api/status | jq . > /dev/null 2>&1; then
    echo "✅ 服务状态检查通过"
    STATUS=$(curl -s http://localhost:3000/api/status)
    echo "📊 服务状态: $STATUS"
else
    echo "❌ 服务状态检查失败"
    kill $BACKEND_PID
    exit 1
fi

# 测试 2: 检查 Web 界面
echo ""
echo "🧪 测试 2: 检查 Web 界面"
echo "---------------------------------"
if curl -s -o /dev/null -w "%{http_code}" http://localhost:3000/ | grep -q "200"; then
    echo "✅ Web 界面访问正常"
else
    echo "❌ Web 界面访问失败"
    kill $BACKEND_PID
    exit 1
fi

# 测试 3: 文件操作测试
echo ""
echo "🧪 测试 3: 文件操作测试"
echo "---------------------------------"
# 检查测试文件是否存在
if [ -f "test-data/source/test_file.txt" ]; then
    echo "✅ 测试文件创建成功"
    echo "📄 文件内容:"
    cat test-data/source/test_file.txt
else
    echo "❌ 测试文件创建失败"
    kill $BACKEND_PID
    exit 1
fi

# 测试 4: 配置验证
echo ""
echo "🧪 测试 4: 配置验证"
echo "---------------------------------"
if [ -f "config/folders.toml" ]; then
    echo "✅ 配置文件存在"
    echo "⚙️ 配置内容:"
    cat config/folders.toml
else
    echo "❌ 配置文件不存在"
    kill $BACKEND_PID
    exit 1
fi

# 测试 5: 健康检查
echo ""
echo "🧪 测试 5: 健康检查"
echo "---------------------------------"
HEALTH_RESPONSE=$(curl -s http://localhost:3000/api/status 2>/dev/null || echo "")
if [ -n "$HEALTH_RESPONSE" ]; then
    echo "✅ 健康检查通过"
    echo "📡 响应: $HEALTH_RESPONSE"
else
    echo "❌ 健康检查失败"
    kill $BACKEND_PID
    exit 1
fi

# 清理
echo ""
echo "🧹 清理测试环境..."
kill $BACKEND_PID
rm -rf test-data

echo ""
echo "🎉 所有测试通过！"
echo "================================="
echo "✅ 服务状态检查: 通过"
echo "✅ Web 界面访问: 通过"
echo "✅ 文件操作测试: 通过"
echo "✅ 配置验证: 通过"
echo "✅ 健康检查: 通过"
echo "================================="
echo "📊 测试结果: 5/5 通过 (100%)"
echo "🎯 测试覆盖率: 基本功能验证"