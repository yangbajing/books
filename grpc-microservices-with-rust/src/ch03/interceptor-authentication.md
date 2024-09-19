# 拦截器和认证

## 拦截器

在 gRPC 服务架构中，拦截器是一个核心概念，它在请求处理流程中扮演着重要角色。拦截器能够在不修改核心业务逻辑的情况下，实现诸如认证、授权和日志记录等横切关注点，从而提高服务的安全性和可维护性。

拦截器主要分为两类：服务端拦截器和客户端拦截器。服务端拦截器在服务器接收到请求后、执行实际处理逻辑之前进行拦截，可以用于请求验证、日志记录等。客户端拦截器则在客户端发送请求之前进行拦截，常用于添加认证信息、请求跟踪等场景。这两种拦截器共同构成了 gRPC 通信中的重要保障机制。

### 服务端拦截器

#### 使用 `tower` 中间件

tonic 基于 Axum 实现，所以 `tower` 生态的各类中间件都可以直接使用。比如：需要打印每个 gRPC 请求的日志，可以使用 `tower-http` 中的 `TraceLayer` 中间件。

添加 `tower-http` 依赖：

```sh
cargo add tracing-subscriber
cargo add tower-http --features trace
```

在 `main.rs` 中添加 `tracing_subscriber` 初始化日志，并添加 `TraceLayer` 中间件：

```rust
// ...
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tracing_subscriber::fmt::init();

  // ...

  Server::builder()
    .layer(TraceLayer::new_for_grpc())
    .add_routes(make_grpc_routes())
    .serve(grpc_addr)
    .await?;

  Ok(())
}
```

使用 `RUST_LOG="tower_http=debug" cargo run` 启动服务，使用 `grpcurl` 发起 gRPC 请求。这时，就可以在终端看到 gRPC 请求的日志：

```sh
2024-09-19T14:19:01.671092Z DEBUG request{method=POST uri=http://localhost:9999/getting.v1.Auth/Signin version=HTTP/2.0}: tower_http::trace::on_request: started processing request
2024-09-19T14:19:01.671302Z DEBUG request{method=POST uri=http://localhost:9999/getting.v1.Auth/Signin version=HTTP/2.0}: tower_http::trace::on_response: finished processing request latency=0 ms
2024-09-19T14:19:01.671420Z DEBUG request{method=POST uri=http://localhost:9999/getting.v1.Auth/Signin version=HTTP/2.0}: tower_http::trace::on_eos: end of stream stream_duration=0 ms status=0
```

可以看到，一个 gRPC 请求的完整生命周期，包括请求开始（`on_request`）、响应结束（`on_response`）、请求结束（`on_eos`）。

> 默认 Rust 将捕获所有日志输出，需要在启动服务时添加 `RUST_LOG="tower_http=debug"` 环境变量才能在终端看到日志输出。Windows 系统需要使用不同的方式设置环境变量。
>
> CMD：
> ```CMD
> set RUST_LOG=tower_http=debug && cargo run
> ```
>
> Powershell：
> ```powershell
> $env:RUST_LOG="tower_http=debug"; cargo run
> ```

#### 使用 tonic interceptor

tonic 还提供了 `tonic::Interceptor` trait，用于在服务端/客户端拦截器中使用。我们对 `UserService` 添加一个 Authentication Interceptor，用于在服务端拦截器中验证请求的认证信息。

定义拦截器函数：

```rust
pub fn auth_interceptor(request: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
  let authorization = request
    .metadata()
    .get("authorization")
    .ok_or_else(|| tonic::Status::unauthenticated("No auth token provided"))?
    .to_str()
    .map_err(|e| tonic::Status::unauthenticated(e.to_string()))?;
  let token = &authorization["Berere ".len()..];
  if token != SESSION_TOKEN {
    return Err(tonic::Status::unauthenticated("Invalid auth token"));
  }
  Ok(request)
}
```

上面代码首先从请求元数据中获取 `authorization` 信息，然后验证 token 是否正确（这里为演示简单，直接使用了一个固定值: `SESSION_TOKEN`）。若 token 验证失败，则返回 `Status::unauthenticated` 错误，tonic 会自动将错误转换为响应给客户端。

要应用拦截器，只需要调用 `UserServer::with_interceptor` 方法，在第2个参数传入 `auth_interceptor` 函数即可：

```rust
UserServer::with_interceptor(UserService, auth_interceptor)
```

这时候再次发起 gRPC 的 `User/Update` 请求，则会收到 `Unauthenticated` 错误：

```sh
grpcurl -plaintext -import-path ./proto \
  -proto getting/v1/user.proto \
  -d '{
    "id": 1,
    "name": "yangbajing"
  }' \
  localhost:9999 getting.v1.User/Update
# 下面是响应输出
ERROR:
  Code: Unauthenticated
  Message: No auth token provided
```

可以通过为 `grpcurl` 命令添加 `-H` 选项，指定 `authorization` 元数据（HTTP Header）。如：`-H 'authorization: Bearer L1AhTRgFMiTkQMuGf8PnY6yHAmaV72ESQsEzo0cVWmiodIEx'`。

### 客户端拦截器

客户端拦截器用于在客户端发送请求时执行一些逻辑。常用的场景是添加认证信息，比如：session token。

我们通过 **example** 来演示客户端拦截器的用法，创建 `examples/client-interceptor.rs` 文件，添加以下 Rust 代码：

```rust
use tonic::{transport::Channel, Code, Request, Status};
use tonic_getting::pb::getting::v1::{user_client::UserClient, UpdateUserRequest};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let channel = Channel::from_static("http://localhost:9999").connect().await?;

  let mut client = UserClient::with_interceptor(channel, |mut request: Request<()>| {
    let token = "Bearer L1AhTRgFMiTkQMuGf8PnY6yHAmaV72ESQsEzo0cVWmiodIEx"
      .parse()
      .map_err(|_| Status::new(Code::Internal, "InvalidMetadataValue"))?;
    request.metadata_mut().insert("authorization", token);
    Ok(request)
  });

  let request = Request::new(UpdateUserRequest { id: 1, name: Some("yangbajing".to_string()), status: Some(100) });

  let response = client.update(request).await?;

  println!("Response: {:?}", response);

  Ok(())
}
```

客户端拦截器也使用了 `with_interceptor` 方法，这里与服务端拦截器不同的是，在第2个参数传入一个闭包函数。当然，也可以像服务端一样定义一个拦截器函数使用。

通过以下命令运行示例程序 `cargo run --example client-interceptor`，可以看到 `User/Update` 请求成功。
