fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Proto files are at workspace root, so go up 3 levels from backend/crates/infra
    let proto_dir = "../../../proto";
    let proto_file = "../../../proto/aisp/v1/chat_orchestrator.proto";

    // Configure prost-build
    let mut config = prost_build::Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos_with_config(config, &[proto_file], &[proto_dir])?;
    Ok(())
}
