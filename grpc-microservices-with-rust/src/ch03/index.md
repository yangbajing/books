# gRPC 和 Rust 快速入门

**本章涵盖**

- 使用 protobuf（protocol buffers）
- 从 `.proto` 文件生成存根
- 在 单独的代码库中维护 `.proto` 文件
- 维护 protobuf 的向后和向前兼容性

两个服务之间的通信就像两个人在交谈：人们使用电话进行连接，而 gRPC 在服务间通信中也起着同样的作用。正如人们使用语言来相互理解，两个服务使用 `protobuf` 来交换消息。选择合适的通信方式对于建立有效的关系至关重要。既然我们理解了这一重要的通信策略，让我们看看 `protobuf` 和 gRPC 如何在微服务通信中一起使用。