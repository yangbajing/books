# Protocol Buffers

 `protobuf` 允许您序列化结构化数据以通过网络传输。您还可以定义服务函数并生成特定语言的源代码。消息和服务函数的定义写在一个名为.proto 的配置文件中，该文件还包含我们将在本书中使用的协议的版本信息（proto3）。还有两个其他版本：proto1，已被弃用，以及 proto2。proto3 发布的主要动机是简化 proto2 中使用的语法。（有关 proto3 和 proto2 的详细比较，请参见 [proto 3 VS proto 2](https://mp.weixin.qq.com/s?__biz=MzA4ODU5NDIyNQ==&mid=2456448049&idx=1&sn=398e2b624169e041f4cc61fb79692aca)。）您在第 2 章中已经看到一个示例消息，所以这次我们深入探讨一下。

## 定义消息类型

```protobuf
syntax = "proto3";

package getting.v1;

enum TokenType {
  TOKEN_TYPE_UNSPECIFIED = 0;
  TOKEN_TYPE_BEARER = 1;
}

message SigninRequest {
  string email = 1;
  string password = 2;
}

message SigninResponse {
  string access_token = 1;
  TokenType token_type = 2;
  UserDto user = 3;
}

message UserDto {
  int64 id = 1;
  string username = 2;
  optional string name = 3;
}
```

- 第一行 `syntax = "proto3";` 描述了 protocol buffer 文件的版本，proto3 是当前最新版本，也推荐在项目中使用此版本。
- `package` 定义了 `protobuf` 文件的命名空间，它有助于避免名称冲突。同时，它也会是生成的 gRPC 服务的 URI 路径部分。
- `enum` 定义了一个枚举类型，它包含两个值， `TOKEN_TYPE_UNSPECIFIED` 和 `TOKEN_TYPE_BEARER`。
- 后面定义了两个消息类型， `SigninRequest` 和 `SigninResponse`。在定义消息字段时，字段类型放在前面，字段的编号放在后面。当 protobuf 消息被序列化时，这此字段数字将被编码进消息中。也就是说，我们只要确保消息类型和编号不变，就可以放心的重命名字段名。
- `optional`（`protobuf v3.14 添加`）标记字段 `name` 可选，它用于在生成的语言特定代码中判断对端是否设置了此变量（Rust 将生成 `Option<String>` 类型，Java 通过 `hasName()` 方法判断）。在二进制、JSON 和 TextFormat 中的表示没有任何区别，也就是说添加或删除 `optional` 标记不会影响序列化后的数据，是兼容的。

### 常用类型

#### 标量值类型（Scalar Value Types）

| .proto 类型 | Rust 类型 | 说明 |
| ---------- | --------- | ---- |
| double     | f64       | 双精度浮点数 |
| float      | f32       | 单精度浮点数 |
| int32      | i32       | 有符号 32 位整数 |
| int64      | i64       | 有符号 64 位整数 |
| bool       | bool      | 布尔值 |
| string     | String    | 字符串 |
| bytes      | Vec<u8>   | 二进制数据 |

完整的标量值类型列表如见官方文档：[https://protobuf.dev/programming-guides/proto3/#scalar](https://protobuf.dev/programming-guides/proto3/#scalar)

### 默认值

- 在 proto3 中，所有标量类型都有默认值：
  - 数值类型默认值为对应 `0` 值
  - bool 默认值为 `false`
  - string 默认值为空字符串
  - bytes 默认值为空字节数组
- enum 默认值为第一个枚举值，且必需为 `0`。
- 消息类型作为字段类型时，默认值为对应的 `Option<T>` 类型，例如 `Option<UserDto>`。
- `repeated` 标记的字段类型默认值为空数组。

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


