fn main() -> std::io::Result<()> {
    prost_build::Config::new()
        .bytes(["."])
        .compile_protos(&["proto/events.proto"], &["proto/"])?;
    Ok(())
}
