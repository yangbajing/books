use crate::pb::getting::{
  v1::{user_server::User, UpdateUserRequest},
  Empty,
};

pub struct UserService;

#[tonic::async_trait]
impl User for UserService {
  async fn update(&self, request: tonic::Request<UpdateUserRequest>) -> Result<tonic::Response<Empty>, tonic::Status> {
    let req = request.into_inner();
    println!("The update user request: {:?}", req);

    Ok(tonic::Response::new(Empty::default()))
  }
}
