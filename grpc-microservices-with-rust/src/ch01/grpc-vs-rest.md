# gRPC 与 REST

REST（Representational State Transfer）是一种广泛采用的微服务协议。然而，如果您有严格的要求，例如低延迟、多语言系统支持等，您可能会考虑使用 gRPC。REST 基于 HTTP 1.x 协议，允许您在客户端和服务器之间以 JSON 或 XML 格式交换消息。另一方面，gRPC 基于 RPC（远程过程调用）架构，使用 `protobuf` 的二进制格式通过 HTTP 2.0 协议交换数据。这并不意味着 REST 与 HTTP 2.0 不兼容；您可以基于该协议设置自定义实现的 REST 服务，使其成为 gRPC 中的内置功能。

由于 gRPC 内置了对 HTTP 2 的支持，您还可以在客户端和服务器之间使用单向和双向流，从而实现高速通信。使用 REST 服务的默认设置，多个客户端与服务器之间的通信可能会导致整体系统性能的延迟。

在某些情况下，REST 比 gRPC 更有利。例如，REST 协议在各种浏览器中都得到支持。由于 gRPC 的支持较少，您可能需要使用代理层，例如 [gRPC Web](https://github.com/grpc/grpc-web)，以便轻松与 gRPC 服务器进行通信。Tonic 提供了 gRPC Web 支持（可以在 [tonic-web](https://github.com/hyperium/tonic/tree/master/tonic-web)了解更多信息），它允许你在一个服务中同时暴露 gRPC 服务端点和 REST 端点，这样就允许你省去部署一个单独的 grpcwebproxy 的开销。
