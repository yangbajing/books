# 何时使用 gRPC

TODO 修改示例！

如果您对浏览器支持有严格要求，那么您需要考虑使用 REST，因为您最终会设置另一个层来在 HTTP/2 和 HTTP/1 之间进行转换。然而，您仍然可以使用 gRPC 进行服务间通信，并将 gRPC 负载均衡器（http://mng.bz/BmZ8）附加到该服务池，以便向公众公开 API 以实现 REST 兼容性，我们将在第 9 章中详细讨论。其他替代方案包括 Twirp（https://github.com/twitchtv/twirp），这是一个基于 Protobuf 构建的 RPC 框架。Twirp 允许您以一种方式为 gRPC 服务启用 REST 层，使您能够访问您的端点，如以下示例所示，该示例发送一个带有 JSON 有效负载的 POST 请求：

```
curl -X "POST" \
      - H "Content-Type: application/json" \
      -d '{"name": "dev-cluster"}' \
      http://localhost:8080/twirp/github.com/huseyinbabal/microservices-proto/cluster/Create
```

多语言开发环境非常适合 gRPC 集成，因为在结账服务中使用 Python 客户端访问用 Java 编写的支付服务非常简单，客户端存根生成使这一过程变得容易。您可以对公共消费者的 SDK 生成应用相同的策略。此外，每当您更改服务定义时，客户端的测试会失败，这为您的微服务提供了合适的验证机制。

您将在第 7 章学习如何测试 gRPC 微服务。对于仅包含一到两个服务的简单应用程序，例如初创项目，gRPC 可能不是合适的选择，因为维护包含服务定义的 proto 文件并不容易，尤其是对于经验不足的用户。

然而，在内部服务之间使用 gRPC 通信是可以接受的，但向客户公开 gRPC 接口可能并不理想，特别是当没有用于 gRPC 服务通信的客户端 SDK 时。如果您希望在不维护消费者 SDK 的情况下公开 gRPC，那么最好与他们共享您的服务定义或提供关于如何向您的 gRPC 服务发起 gRPC 调用的清晰说明。

## 本书适合谁？

本书包含许多解释、代码示例以及由真实案例支持的技巧和窍门，这些对以下角色可能会有用：

- 不懂 Rust 或微服务的开发者：他们可以从关于 Rust、微服务和 gRPC 的入门章节开始，学习 gRPC Rust 微服务的生产级技术。已经了解微服务架构的读者可以通过 Rust 中描述的资源来刷新他们的知识，这些资源可以轻松适应其他任何语言。
- 工程经理：他们可以通过在其手册中添加最佳实践来提高团队开发人员的生产力。应用这些技术将为整个产品引入良好的可见性，这将有助于快速让新员工融入团队。
- 软件架构师：有许多方便的示例和架构设计可以作为他们在新产品或功能决策时的潜在参考。

