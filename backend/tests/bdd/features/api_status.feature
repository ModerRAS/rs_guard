# language: zh-CN
功能: API 状态检查

  作为一个系统管理员
  我想要检查系统的运行状态
  以便了解系统是否正常工作

  场景: 获取系统状态
    假设 应用已启动
    当 发送 GET 请求到 /api/status
    那么响应状态应该是 200
    并且响应应该包含字段 data_shards
    并且字段 data_shards 应该是 4
    并且响应应该包含字段 parity_shards
    并且字段 parity_shards 应该是 2
    并且响应应该包含字段 watched_dirs
    并且字段 watched_dirs 应该是一个数组