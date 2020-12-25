fn main() {
    prost_build::compile_protos(&["proto/error_details.proto"], &["proto/"]).unwrap();
}