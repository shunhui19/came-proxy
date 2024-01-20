详细设计说明：
一、模块划分
    * loader balance 负载均衡模式
    * protocol 协议模块 
    * health checking 健康检查模块
    * metrics 数据指标模块
    * common 公共模块
二、代码结构
|--common
|--config
|--protocols
|----http.rs
|--



三、每个模块详细设计
1.protocol module
支持http1.1/1.2, grpc, websocket多种协议
应该抽象出一个trait，提炼通用方法出来, 方便后续扩展
