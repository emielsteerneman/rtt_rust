fn main() {
    // It compiles to OUT_DIR because that way it stays deterministic / hermetic.
    // The files are generated to the file protos.rs in the OUT_DIR. I don't know why.
    // Can I specify the output file? Why are both .proto files thrown together into a single output file? Will this break stuff?
    // ./target/debug/build/protos-<hash>/out

    // Correction, now with the SSL stuff its suddenly _.rs ???
    prost_build::compile_protos(
        &[
            "proto/ssl_vision_wrapper.proto",
            "proto/ssl_gc_referee_message.proto",
        ],
        &["proto"],
    )
    .expect("Failed to compile Protobuf files");
}
