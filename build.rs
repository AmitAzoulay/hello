fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    unsafe {
        std::env::set_var("PROTOC", protoc);
    }


    println!("cargo:warning=*** build.rs is running and compiling protos ***");

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(&["proto/hello.proto"], &["proto"])?;

    println!("cargo:rerun-if-changed=proto/hello.proto");
    println!("cargo:rerun-if-changed=proto");
    Ok(())
}
