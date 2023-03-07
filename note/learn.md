- Web Server
  - Server，监听进来的TCP字节流
  - Router，接受HTTP请求，并决定调用哪个Handler
  - Handler，处理HTTp请求，构建HTTP响应
  - HTTP Library
    - 解释字节流，把它转化为HTTP请求
    - 把HTTP响应转化为字节流

- 构建步骤：
  - 解析HTTP请求
  - 构建HTTP请求
  - 路由与Handler
  - 测试Web Server

- 数据结构
- HttpRequest struct
- Method enum
- Version enum

- 三个数据结构要实现三个Trait
- From<&str>
- Debug
- PartialEq

- HttpResponse需要实现的方法or trait
  - Default trait
  - new()
  - send_response()
  - getter方法
  - From trait
