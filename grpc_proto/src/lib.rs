

use crate::pb::hello_service_server::HelloServiceServer;

pub mod pb {
    include!("./helloworld/helloworld.rs");
    pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("./helloworld/helloworld_descriptor.bin");
}



