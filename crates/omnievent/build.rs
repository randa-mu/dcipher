fn main() -> std::io::Result<()> {
    tonic_build::configure().bytes(["."]).compile_protos(
        &["../../dcipher-proto/omnievent/events.proto"],
        &["../../dcipher-proto/omnievent/"],
    )?;
    Ok(())
}
