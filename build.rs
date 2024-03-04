use protobuf_codegen::Codegen;

// Use this in build.rs
fn main() {
    Codegen::new()
        .pure()
        .out_dir("src")
        .include("src")
        .input("src/protos/profile.proto")
        .run_from_script();
}