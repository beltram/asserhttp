fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(any(feature = "grpc", feature = "tonic"))]
    for proto in std::fs::read_dir("tests/protos").unwrap() {
        proto.map(|p| p.path()).ok().map(tonic_build::compile_protos).transpose()?;
    }
    Ok(())
}
