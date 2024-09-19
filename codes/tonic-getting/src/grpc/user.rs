use crate::pb::getting::{
  v1::{user_server::User, UpdateUserRequest},
  Empty,
};

pub struct UserService;

#[tonic::async_trait]
impl User for UserService {
  async fn update(&self, request: tonic::Request<UpdateUserRequest>) -> Result<tonic::Response<Empty>, tonic::Status> {
    println!("The update user request: {:?}", request);

    Ok(tonic::Response::new(Empty::default()))
  }
}
