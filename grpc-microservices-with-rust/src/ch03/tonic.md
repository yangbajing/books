# 使用 tonic

gRPC Stub 是一个作为 gRPC 客户端接口的模块。您可以使用这些存根执行多项操作，例如通过流式或非流式表示法连接和交换数据。 `protobuf` 编译器为指定语言生成源代码，而该源代码包含所有存根。您可以将生成的源代码导入客户端和服务器端，以通过遵循接口中定义的合同来实现业务逻辑。

## 创建项目

### 初始化配置 gRPC 项目

打开命令行终端，依次输入以下指令创建 gRPC 项目：

```sh
cargo new tonic-getting
cd tonic-getting
# 若找不到 cargo `add` 命令请安装：`cargo install cargo-edit`。
cargo add tonic prost anyhow
cargo add --build tonic-build
```

### 创建 proto 文件

创建 `proto` 目录存放 protocol buffers 文件定义。添加以下内容到文件 `proto/getting/v1/auth.proto` 中：

```protobuf
syntax = "proto3";

package getting.v1;

service AuthService {
  rpc Signin(SigninRequest) returns (SigninResponse) {}
  rpc Signup(SignupRequest) returns (SignupResponse) {}
}

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
}

message SignupRequest {
  string email = 1;
  string password = 2;
}

message SignupResponse {
}
```

### 使用 `build.rs` 自动化编译 proto 存根代码

创建 `build.rs` 文件自动编译 `*.proto` 并生成 Rust 存根代码。

```rust
fn main() {
    println!("cargo::rerun-if-changed=proto/getting/**/*");

    tonic_build::configure()
        .compile(&["proto/getting/v1/auth.proto"], &["proto"])
        .unwrap();
}
```

正确完成这些步骤后，`cargo` 将自动编译 .proto 文件并生成 Rust 存根代码，生成的代码默认并存放在 `target/debug/build/tonic-getting-bbf494a1a6da115c/out/getting.v1.rs`（注：你看到的具体的路径可能不一样，注意16进制字符串那部分可能会不同）。

### 测试 protocol buffer 消息

创建 `src/lib.rs` 文件，并添加如下内容：

```rust
pub mod pb;
```

创建对应的 `pb`（文件：`src/pb.rs`）模块，并添加如下内容：

```rust
pub mod getting {
    pub mod v1 {
        tonic::include_proto!("getting.v1");
    }
}
```

修改 `main.rs` 文件如下：

```rust
use tonic_getting::pb::getting::v1::SigninRequest;

fn main() {
    let signin_req = SigninRequest {
        email: "yangbajing@gmail.com".to_string(),
        password: "Password.2024".to_string(),
    };
    println!("Signin request is {:?}", signin_req);
}
```

执行命令 `cargo run -q` 运行程序，可以正常输出由 protobuf 定义的消息结构体 `SigninRequest` 的内容。

```sh
Signin request is SigninRequest { email: "yangbajing@gmail.com", password: "Password.2024" }
```

### 配置 `rust-analyzer` 分析 `build.rs` 生成的文件

默认情况下，`rust-analyzer` 不会分析 `build.rs` 生成的文件（更准确的说是不会解析在标准源码目录之外的 `.rs` 文件，如：`src`、`examples`、`test`）。可以通过修改 `.vscode/settings.json` 文件添加 VSCode 配置来启用对 `build.rs` 生成的文件的分析。

```json
{
  "rust-analyzer.cargo.buildScripts.enable": true,
}
```

重启 `rust-analyzer server` 后，就可以通过 `F2`（或者 `Ctrl/CMD + 鼠标点击`）跳转到生成的代码定义位置。

## 定义 gRPC 服务

接下来定义 gRPC 服务并启动它。我们先创建一个 `src/grpc.rs` 模块，并添加如下代码：

```rust
use crate::pb::getting::v1::{
    auth_server::Auth, SigninRequest, SigninResponse, SignupRequest, SignupResponse, TokenType,
};

pub struct AuthService;

#[tonic::async_trait]
impl Auth for AuthService {
    async fn signin(
        &self,
        request: tonic::Request<SigninRequest>,
    ) -> Result<tonic::Response<SigninResponse>, tonic::Status> {
        let req = request.into_inner();
        println!("Signin request is {:?}", req);

        let resp = SigninResponse {
            access_token: "".to_string(),
            token_type: TokenType::Bearer as i32,
        };
        Ok(tonic::Response::new(resp))
    }

    async fn signup(
        &self,
        request: tonic::Request<SignupRequest>,
    ) -> Result<tonic::Response<SignupResponse>, tonic::Status> {
        let req = request.into_inner();
        println!("Signup request is {:?}", req);

        let resp = SignupResponse {
            code: 0,
            ..Default::default()
        };
        Ok(tonic::Response::new(resp))
    }
}
```

添加 `tokio` 依赖：

```sh
cargo add tokio --features full
```

然后再次修改 `main.rs` 文件，添加 gRPC 启动服务：

```rust
use tonic::transport::Server;
use tonic_getting::{grpc::AuthService, pb::getting::v1::auth_server::AuthServer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let grpc_addr = "0.0.0.0:9999".parse()?;
  println!("The gRPC Server listening to {}", grpc_addr);
  Server::builder()
    .add_service(AuthServer::new(AuthService)) // 添加 Auth gRPC Service
    .serve(grpc_addr).await?; // 绑定到指定网络地址并启动 gRPC 服务
  Ok(())
}
```

我们在命令行终端输入 `cargo run -q` 启动服务。然后再打开另一个命令行终端，并使用 `grpcurl` 来测试服务：

```sh
grpcurl -plaintext -import-path ./proto \
  -proto getting/v1/auth.proto \
  -d '{
    "email": "yangbajing@gmail.com",
    "password": "Password.2024"
  }' \
  localhost:9999 getting.v1.Auth/Signin
# 下面是输出内容。
{
  "accessToken": "L1AhTRgFMiTkQMuGf8PnY6yHAmaV72ESQsEzo0cVWmiodIEx",
  "tokenType": "TOKEN_TYPE_BEARER"
}
```

## Protobuf package 与 Rust mod 的映射关系

在示例中，proto 定义在了 `getting.v1` package 中，那么生成的 Rust 代码就一定位于 `getting::v1` module 中吗？实际上是不会的，`tonic` 生成的代码并不会包含 `getting::v1` 这个父模块路径，这些都需要我们自己定义。我们再创建几个 .proto 文件看看 `tonic` 生成的代码就会明白了。

创建 `proto/getting/common/page.proto` 文件并添加如下内容：

```protobuf
syntax = "proto3";

package getting.common;

message Pagination {
  int64 page = 1;
  int64 page_size = 2;
}
```

创建 `proto/getting/v1/user.proto` 文件并添加如下内容：

```protobuf
syntax = "proto3";

package getting.v1;

import "getting/common/page.proto";

message UserDto {
  int64 id = 1;
  string email = 2;
  optional string name = 3;
  int32 status = 4;
}

message PageUserRequest {
  getting.common.Pagination pagination = 1;
}
```

相应的修改 `build.rs` 文件，将 `"proto/getting/v1/user.proto", "getting/common/page.proto"` 添加到 `compile` 方法的第一个参数数组里（`protos`）以让 `tonic-build` 可以编译相关的 .proto 文件。

然后我们在 `pb.rs` 模块里写个简单的测试：

```rust
#[cfg(test)]
mod tests {
  use super::getting::v1::*;

  #[test]
  fn test_user() {
    let pagination = Pagination { page: 1, page_size: 20, ..Default::default() };
    let page_user_request = PageUserRequest { pagination: Some(pagination) };
    println!("Page user request is {:?}", page_user_request);
  }
}
```

并通过 `cargo test pb::tests -q` 命令运行测试，这时会输出编译错误：

```sh
error[E0433]: failed to resolve: could not find `common` in `super`
   --> /Users/yangjing/workspaces/grpc-microservices-with-rust/tonic-getting/target/debug/build/tonic-getting-caf65bf194fbeca1/out/getting.v1.rs:409:51
    |
409 |     pub pagination: ::core::option::Option<super::common::Pagination>,
    |                                                   ^^^^^^ could not find `common` in `super`

error[E0422]: cannot find struct, variant or union type `Pagination` in this scope
  --> src/pb.rs:18:22
   |
18 |     let pagination = Pagination { page: 1, page_size: 20, ..Default::default() };
   |                      ^^^^^^^^^^ not found in this scope
```

可以看到，在生成的 `getting.v1.rs` 文件中，`pagination` 字段对 `Pagination` 类型的应用使用了 `super::common::Pagination` 路径，而 `super` 在模块树中指的是上级目录，所以这里会报错。那我们在 `v1` 同级模块添加 `pub mod common` 模块代码，但这时 `tonic::include_proto!` 我们应该引入哪个文件呢？我们定位到 `getting.v1.rs` 所在目录会发现目录内文件如下：

```sh
.
├── getting.common.rs
├── getting.v1.rs
```

看到有 `getting.common.rs` 文件，我们尝试在 `pb.rs` 文件中引入 `getting::common::*` 模块，并修改测试代码：

```rust
pub mod getting {
  pub mod common {
    tonic::include_proto!("getting.common");
  }
  pub mod v1 {
    tonic::include_proto!("getting.v1");
  }
}

#[cfg(test)]
mod tests {
  use super::getting::common::*;
  use super::getting::v1::*;

  // ... 测试代码
}
```

重新运行测试，可以看到测试通过了。

`tonic-build` 对于 protobuf 的 `package` 路径，会生成以 `.`（英文点号）分隔的相应 rust 文件。如 `package getting.common` 会生成 `getting.common.rs` 文件，`package getting.v1` 会生成 `getting.v1.rs` 文件。而 Rust 代码里面对其它模块类型的引用，则使用 `super::common::Pagination` 这种相对路径的形式。所以我们在 `pb.rs` 文件里需要按 proto `package` 的路径来组织 Rust mod 层次。

> 你可以试着在 `proto` 目录下创建一个 `basic.proto` 文件，并指定 `package getting`。这时，当在 `user.proto` 文件中引用 `basic.proto` 文件中的类型时，生成的 Rust 文件会是 `getting.rs`，这时在 `pb.rs` 文件中引用 `getting.rs` 文件中的类型时，就需要使用 `super::super::BasicType` 这种形式了。

## 小结

### 创建 Rust gRPC 服务快速步骤

#### 这一个简单的 gRPC 服务就完成了，让我们总结下使用 Rust 开发 gRPC 服务的步骤

1. 使用 `cargo new` 命令创建项目
2. 添加必要的依赖，`tonic`、`prost`、`tonic-build`、`tokio`
3. 引入 `.proto` 文件，通常放到 proto 目录中
4. 创建 `build.rs` 文件，并添加 `tonic-build` 定义
5. 创建 `pb` 模块，并通过 `tonic::include_proto!` 宏导入生成的 protobuf 存根 Rust 代码
6. 创建 gRPC 服务（使用 tonic）

#### Protobuf `package` 路径与 Rust `mod` 路径的映射关系

- `package` 会生成以 `.`（英文点号）分隔的相应 rust 文件
  - 如：`package getting.common` 会生成 `getting.common.rs` 文件，`package getting.v1` 会生成 `getting.v1.rs` 文件。
- 在 `pb.rs` 文件中，对 `package` 中定义的类型的引用，使用 `super::common::Pagination` 这种相对路径的形式。

#### 定义服务

- 定义 `XxxService` 结构体，并实现 `Xxx` trait
- 使用 `Server::builder().add_service(XxxServer::new(XxxService)).serve(addr)` 启动服务
