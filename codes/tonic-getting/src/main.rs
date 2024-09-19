use tonic::transport::Server;
use tonic_getting::grpc::make_grpc_routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let grpc_addr = "0.0.0.0:9999".parse()?;
  println!("The gRPC Server listening to {}", grpc_addr);
  Server::builder().add_routes(make_grpc_routes()).serve(grpc_addr).await?;
  Ok(())
}
