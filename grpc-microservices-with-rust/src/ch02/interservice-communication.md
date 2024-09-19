# 使用 gRPC 进行服务间通信

gRPC 是一种现代的、轻量级的通信协议和高性能的 RPC 框架，由 Google 引入。它可以有效地连接微服务环境中的服务，内置支持负载均衡、追踪、健康检查和身份验证。gRPC 使用 `protobuf` 提供易于使用和高效的通信， `protobuf` 是一种用于序列化结构化数据的开源机制。让我们使用自动生成的 Rust 代码来考虑使用 gRPC 和 `protobuf` 使服务相互通信（即交换消息）的最小步骤。

## 使用 protobuf

让我们重新审视订单创建流程，以使用 gRPC 和 `protobuf` 进行服务间通信。理想的步骤如下：

1. 定义包含消息和服务描述的 `proto` 文件。这些文件可以在当前项目中定义，也可以在一个单独的代码库中进行维护。
2. 从 `.proto` 文件生成客户端和服务端存根。
3. 通过使用支持的语言之一（https://www.grpc.io/docs/languages/）实现服务端业务逻辑。
4. 实现客户端业务逻辑，并通过客户端存根连接服务。
5. 运行服务端和客户端。

任何 `.proto` 文件都有消息定义和服务函数的共同点。消息可能指的是请求对象、响应对象，以及微服务环境中典型服务常用的枚举。例如，支付服务将有一个 `CreatePaymentRequest` 消息供订单服务使用。同样， `CreatePaymentResponse` 可以返回 `bill_id` 给订单服务，以将其作为订单服务实体中的一个单独字段存储。仅有这些消息是不够的，因此我们需要定义服务以使用这些请求和响应消息。 `Create` 服务函数可以将 `CreatePaymentRequest` 作为函数参数，并返回 `CreatePaymentResponse` 作为函数返回签名。这些服务函数的输入和输出参数很重要，因为它们直接影响客户端的可用性：

```protobuf
message CreatePaymentRequest {
    int user_id = 1;
    float64 price = 2;
}

message CreatePaymentResponse {
    int bill_id = 1;
}

service Payment {
    RPC Create(CreatePaymentRequest)
    returns (CreatePaymentResponse) {}
}
```

Payment 和 Shipping 存根可以通过 `protobuf` 编译器为多种语言轻松生成，以便任何服务，例如订单，可以像图 2.8 所示那样使用它们 , 。例如，订单服务可以在 Payment 存根中调用 Create 函数，该函数使用 gRPC 协议访问 Payment 中的实际服务端点。 protoc 用于从 `protobuf` 描述文件生成 Go 源代码。在这种情况下， CreatePaymentRequest 和 CreatePaymentResponse 消息将被编译为 Rust 结构体，以用于数据交换操作，进行请求和响应的序列化/反序列化（图 2.8）。在这里，序列化意味着将 Golang 结构体转换为字节数组。您可以在这里查看内部实现：http://mng.bz/AoVK。

TODO 补充图示

现在我们了解了如何使用 `protobuf` 来定义服务合同（第 3 章将有更深入的讨论），让我们看看如何通过使用 .proto 文件生成源代码。

## 生成 Rust 代码

`protobuf` 编译器（`protc`），是一个使用 `.proto` 文件生成特定语言的客户端和服务端实现的工具。我们将在第 3 章详细讨论 `.proto` 文件的使用，现在假设以下支付服务的 `proto` 定义：

```protobuf
syntax = "proto3";                        ❶

package microservice.v1;                  ❷

message CreatePaymentRequest {
    float price = 1;
}

message CreatePaymentResponse {
    int64 bill_id = 1;
}

service Payment {
    RPC Create(CreatePaymentRequest) returns (CreatePaymentResponse) {}
}
```

1. `protobuf` 版本
2. protobuf 包名，消息和服务均在此包名空间下

可以使用 [tonic-build](https://crates.io/crates/tonic-build) 来自动化生成该 `.proto` 文件的客户端和服务端 Rust 代码。

```toml
[dependencies]
tonic = { version = "0.12" }

[dev-dependencies]
tonic-build = { version = "0.12" }
```

在项目根目录下创建 `build.rs` 文件，并添加以下内容：

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("src/ch02/payment.proto")?;
    Ok(())
}
```

运行 `cargo build` 时，会自动运行 `build.rs` 文件，生成 `src/ch02/payment.rs` 文件。

TODO 修改并完善 cargo 项目配置

这个示例是一个简单的请求/响应风格，称为单一 RPC，将生成所有的 Rust 代码并将其放置在相对于输入 .proto 文件的位置。gRPC 还允许您在客户端和服务器端使用流式传输。要实现流式响应，您可以在 .proto 文件中修改您的服务函数：

```protobuf
service Payment {
    RPC CreateStream(CreatePaymentRequest) returns (stream CreatePaymentResponse) {}
}
```

流式传输允许服务器端将数据拆分并逐部分返回给客户端。服务器根据客户端的请求返回一系列 `CreatePaymentResponse` 。一旦服务器发送完所有消息，它还会向客户端发送一些元数据，以表明服务器端的流式操作已完成。客户端在看到此状态后完成其操作。

服务器端流式传输在服务器需要向客户端返回大量数据时是有益的。流式传输也可以在客户端发生。例如，支付请求以流式模式发送到服务器。服务器不需要发送多个消息，而是可以返回一条消息，报告所有支付创建操作的响应及其状态：

```protobuf
service Payment {
    RPC CreateStream(stream CreatePaymentRequest) returns (CreatePaymentResponse) {}
}
```

双向流式传输是指在客户端和服务器端同时进行流式传输。它允许客户端持续向服务器发送请求，服务器可以以对象流的形式返回结果。由于该操作是异步处理的，因此最好提供一个唯一标识符来标记哪个操作失败。

```protobuf
service Payment {
    RPC CreateStream(stream CreatePaymentRequest) returns (stream CreatePaymentResponse) {}
}
```

我们将在第 5 章详细讨论这个概念。

## 连接服务

client-server 通信意味着在微服务环境中的服务到服务通信。服务是一个典型的消费者，使用另一个服务的客户端存根。在我们的案例中，如果您想将订单服务连接到支付服务，请完成以下步骤：

1. 将 payment 服务客户端存根作为领导导入 order 服务。
2. 通过从 order 服务连接 payment 服务。我们可以通过服务发现来获取 payment 服务的地址。
3. 使用创建的连接对象创建一个 payment 客户端。
4. 调用 `Payment` 客户端上的 `Create` 方法。

```rust
// TODO 补充代码清单
```

在列表 2.2 中，我们初始化一个连接到地址并将其作为参数传递给 `PaymentClient` 。请注意，我们使用 Rust 的 RAII 机制（`Drop`）以确保在应用程序关闭后连接被正确关闭。我们不关心在连接到两个服务时底层通信是如何处理的。一切都被抽象为一个客户端实例，该实例由 `tonic-build` 自动生成到 Rust 中。

 `protobuf` 编译器还生成了一个客户端实现，命名约定为 New<Service_Name>Client ，这样您就可以为特定服务的客户端创建一个新的引用（例如， NewPaymentClient ）。当您调用客户端存根时，它可以返回成功或失败。为了实现可靠的通信，最好根据某些标准重试失败的请求。同样，最好为此执行提供超时，以便在客户端无法在请求的时间间隔内看到响应时使操作失败。这些最佳实践都是为了使数据更加一致。如果您有一个处于待处理状态的订单对象，它在那里停留了几个小时，这就是不正确的通信模式的标志。正如您在最后一个示例中看到的，尽管我们没有在 .proto 文件中明确定义，但仍然有一个 ctx 参数。Protobuf 编译器只是向所有服务函数添加了一个上下文参数，以允许消费者传递其上下文对象的引用。例如，如果您在 10 秒内没有获得结果，可以使用以下代码取消执行：

TODO 上面需要更新为 Rust 代码示例
