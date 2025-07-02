fn main() -> std::io::Result<()> {
    tonic_build::configure()
        .bytes(["."])
        .compile_protos(&["proto/events.proto"], &["proto/"])?;
    Ok(())
}
