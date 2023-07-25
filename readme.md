# example-kv-server

## 需要的功能

1. 从K8s的configmap中读取到配置文件
2. 提供proto-reflection的服务
2. 连接并注册到consul，consul的地址从配置文件中读取
3. 连接到redis，可以正常的实现set redis
