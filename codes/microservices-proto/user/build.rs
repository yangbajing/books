use std::{env, path::PathBuf};

fn main() {
  println!("cargo::rerun-if-changed=proto/**/**/*.proto");

  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

  tonic_build::configure()
    .file_descriptor_set_path(out_dir.join("microservices_user_descriptor.bin"))
    .compile(&["proto/microservices_user/v1/user.proto"], &["proto"])
    .unwrap();
}
