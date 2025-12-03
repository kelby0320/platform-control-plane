pub mod client;
pub mod mapper;

// Shared proto module - generated code included once
pub mod proto {
    pub mod aisp {
        pub mod v1 {
            tonic::include_proto!("aisp.v1");
        }
    }
}
