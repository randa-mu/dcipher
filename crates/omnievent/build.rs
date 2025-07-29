fn main() -> std::io::Result<()> {
    tonic_build::configure().bytes(["."]).compile_protos(
        &["../../protobuf/omnievent/events.proto"],
        &["../../protobuf/omnievent/"],
    )?;
    Ok(())
}
