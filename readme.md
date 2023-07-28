# example-kv-server


## 自动化部署
使用Github Action 功能

## 静态检查

使用clippy进行静态检查, use command : `cargo clippy`， 并将其集成到Github Action中

## 静态配置读取

从K8s的configmap中读取到配置文件

## Trace接入
接入Trace追踪，you can refer to [opentelemetry-rust](https://docs.rs/opentelemetry-otlp/0.12.0/opentelemetry_otlp/)
## 服务注册
连接并注册到consul，consul的地址从配置文件中读取,并将自己注册在consul上

## others
提供proto-reflection的服务, 可以通过grpcurl进行调用, use command : `grpcurl -plaintext localhost:50051 list`

## 正常的功能调用
3. 连接到redis，可以正常的实现set redis
