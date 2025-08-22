fn main() -> std::io::Result<()> {
    tonic_build::configure()
        .server_mod_attribute("attrs", "#[cfg(feature = \"grpc-server\")]")
        .client_mod_attribute("attrs", "#[cfg(feature = \"client\")]")
        .bytes(["."])
        .compile_protos(
            &["../dcipher-proto/dsigner/dsigner.proto"],
            &["../dcipher-proto/dsigner/"],
        )?;
    Ok(())
}
