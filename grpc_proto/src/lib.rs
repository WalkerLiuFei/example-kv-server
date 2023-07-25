pub mod pb {
    include!("./helloworld/helloworld.rs");
    // with this line, the compiler will complain about the following:
    pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("./helloworld/helloworld_descriptor.bin");
}



