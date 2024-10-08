# Summary

[前言](./preface.md)

# Part 1. gRPC 和微服务架构

- [gRPC](./ch01/index.md)
  - [gRPC 微服务的优势](./ch01/benefit.md)
  - [gRPC 与 REST](./ch01/grpc-vs-rest.md)
  - [gRPC 与 GraphQL]()
  - [何时使用 gRPC](./ch01/when-use-grpc.md)
  - [生产级用例](./ch01/production-use-case.md)
  - [总结](./ch01/summarize.md)
- [微服务](./ch02/index.md)
  - [单体架构](./ch02/monolithic.md)
  - [微服务架构](./ch02/microservices.md)
  - [服务发现](./ch02/service-discovery.md)
  - [使用 gRPC 进行服务间通信](./ch02/interservice-communication.md)
  - [总结](./ch02/summarize.md)
- [gRPC 和 Rust 快速入门](./ch03/index.md)
  - [Protocol Buffers](./ch03/protobuf.md)
  - [使用 tonic](./ch03/tonic.md)
  - [拦截器和认证](./ch03/interceptor-authentication.md)
  - [维护 .proto 文件](./ch03/maintain-proto-file.md)
  - [总结]()

# Part 2. 开发、测试和部署 gRPC 微服务

- [微服务项目设置]()
  - [六边型架构]()
  - [订单服务]()
  - [总结]()
- [服务间通信]()
  - [gRPC 服务间通信]()
  - [其它服务：产品、支付]()
  - [错误处理]()
  - [总结]()
- [弹性通信]()
  - [弹性模式]()
  - [深入错误处理]()
  - [保护 gRPC 通信]()
  - [总结]()
- [测试微服务]()
  - [单元测试]()
  - [集成测试]()
  - [端到端测试]()
  - [总结]()
- [部署]()
  - [Docker]()
  - [Kubernetes]()
  - [证书管理]()
  - [部署策略]()
  - [总结]()

# Part 3. 更进一步

- [gRPC 生态]()
  - [gRPC-Web](./ch09/gRPC-Web.md)
  - [结合 Next.js](./ch09/nextjs.md)
  - [总结]()
- [可观测性]()
  - [日志]()
  - [指标]()
  - [跟踪]()
  - [总结]()

# 附录

- [附录](./appendix/index.md)
  - [附录 A：proto3 兼容性](./appendix/proto3-compatibility.md)
  - [附录 B：tonic-build 编译手册](./appendix/tonic-build.md)
