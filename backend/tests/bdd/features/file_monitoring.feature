# language: zh-CN
功能: 文件监控

  作为一个数据保护系统
  我想要监控文件系统的变化
  以便自动保护新创建的文件

  背景:
    假设 应用已启动

  场景: 创建新文件被监控
    假设 在监控目录中创建测试文件 test1.txt
    当 等待 1000 毫秒
    那么文件 test1.txt 应该存在
    并且文件 test1.txt 应该包含内容 This is test file test1.txt

  场景: 创建多个文件
    假设 在监控目录中创建测试文件 test2.txt
    并且 在监控目录中创建测试文件 test3.txt
    当 等待 1000 毫秒
    那么文件 test2.txt 应该存在
    并且文件 test3.txt 应该存在

  场景: 大文件监控
    假设 在监控目录中创建测试文件 large_file.dat
    当 等待 2000 毫秒
    那么文件 large_file.dat 应该存在