# example-kv-server


## 自动化部署
使用Github Action 功能

## 静态检查

使用clippy进行静态检查, use command : `cargo clippy`， 并将其集成到Github Action中

2. 从K8s的configmap中读取到配置文件
2. 提供proto-reflection的服务
2. 连接并注册到consul，consul的地址从配置文件中读取
3. 连接到redis，可以正常的实现set redis
