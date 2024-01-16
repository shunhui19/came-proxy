came proxy

features
* reverse proxy
* load balancer
    1. round robin, round by time sort
    2. weight round robin
    3. random
* protocol
    * http
    * https
    * websocket
    * grpc
* health checking 健康检查通过以下几种方式：
    * max_fails: 在指定时间间隔[fail_timeout]内最大失败的请求数，超过这个数认为服务不健康
    * response_time: 上游响应时间
    * health_check: 主动进行健康检查
* metrics
    1. 流量指标：如每秒请求数(QPS), 并发连接数
    2. 延迟指标：平均响应时间、延迟分布
    3. 错误指标：不同类型的错误次数
    4. 资源指标：CPU、内存、网络
