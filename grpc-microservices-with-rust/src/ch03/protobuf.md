# Protocol Buffers

 `protobuf` 允许您序列化结构化数据以通过网络传输。您还可以定义服务函数并生成特定语言的源代码。消息和服务函数的定义写在一个名为.proto 的配置文件中，该文件还包含我们将在本书中使用的协议的版本信息（proto3）。还有两个其他版本：proto1，已被弃用，以及 proto2。proto3 发布的主要动机是简化 proto2 中使用的语法。（有关 proto3 和 proto2 的详细比较，请参见 https://www.hackingnote.com/en/versus/proto2-vs-proto3。）您在第 2 章中已经看到一个示例消息，所以这次我们深入探讨一下。

## 定义消息类型

TODO

## protobuf 编码

 `protobuf` 编码的主要目标是将 .proto 文件内容转换为二进制格式，以便通过网络发送。 `protobuf` 编译器使用一组规则将消息转换为二进制格式，以便在数据的封送（序列化）、发送（通过网络）和解封送（反序列化）过程中获得更好的性能。让我们分析下面的示例，看看 `protobuf` 编码在背后是如何工作的。

该 CreateOrderRequest 消息只有一个字段， user_id ，类型为 int , ，字段编号为 1 。我们编译了该消息并在我们的生产代码中使用了它：

```protobuf
// order.proto
message CreateOrderRequest {
    int64 user_id = 1;
}
```

```rust
TODO
```


