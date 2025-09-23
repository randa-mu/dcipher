fn main() -> std::io::Result<()> {
    tonic_build::configure().bytes(["."]).compile_protos(
        &["../../modules/dcipher-proto/omnievent/events.proto"],
        &["../../modules/dcipher-proto/omnievent/"],
    )?;
    Ok(())
}
