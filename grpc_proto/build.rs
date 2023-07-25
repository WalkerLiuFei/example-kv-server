

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true) // Generates gateway
        .build_server(true) // Generates server
        .out_dir("src/helloworld")
        // this is the path to the descriptor file, decscriptor file is needed for gateway and reflection
        .file_descriptor_set_path("src/helloworld/helloworld_descriptor.bin")
        .compile(&["proto/helloworld.proto"], &["proto"])?;
    Ok(())
}