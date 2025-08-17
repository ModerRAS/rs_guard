# language: zh-CN
功能: 数据保护

  作为一个数据保护系统
  我想要对文件进行编码和冗余存储
  以便在数据损坏时能够恢复

  背景:
    假设 应用已启动
    并且 在监控目录中创建测试文件 protected_file.txt

  场景: 文件被编码保护
    当 等待 2000 毫秒
    那么文件 protected_file.txt 应该存在
    并且发送 GET 请求到 /api/files
    那么响应状态应该是 200
    并且响应应该包含字段 files

  场景: 检查文件完整性
    当 发送 GET 请求到 /api/check
    那么响应状态应该是 200
    并且响应应该包含字段 status
    并且字段 status 应该是 healthy

  场景: 获取受保护文件列表
    当 发送 GET 请求到 /api/files
    那么响应状态应该是 200
    并且响应应该包含字段 files
    并且字段 files 应该是一个数组