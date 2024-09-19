use tonic::service::{Routes, RoutesBuilder};

use crate::pb::getting::v1::{auth_server::AuthServer, user_server::UserServer};

mod auth;
mod user;

pub use auth::AuthService;
pub use user::UserService;

pub fn make_grpc_routes() -> Routes {
  let mut rb = RoutesBuilder::default();
  rb.add_service(AuthServer::new(AuthService));
  rb.add_service(UserServer::new(UserService));
  rb.routes()
}
