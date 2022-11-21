use std::{path::PathBuf, env};

fn main () -> Result<(), Box<dyn std::error::Error>> {
    let descriptor_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("general_descriptor.bin");
    tonic_build::configure()
        .file_descriptor_set_path(&descriptor_path)
        .compile(&["protos/general.proto"], &["protos/"])?;
    Ok(())
}
