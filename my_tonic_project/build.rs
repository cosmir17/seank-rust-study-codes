use std::error::Error;
use std::{env, path::PathBuf};
use config_struct::StructOptions;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("service_descriptor.bin"))
        .compile(&["proto/service.proto"], &["proto"])?;

    tonic_build::compile_protos("proto/service.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));

    config_struct::create_struct(
        "config.toml",
        "src/config.rs",
        &StructOptions::default()).unwrap();

    Ok(())
}
