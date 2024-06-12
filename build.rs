use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from("src/proto_gen");

    // Create the output directory if it doesn't exist
    std::fs::create_dir_all(&out_dir).unwrap();

    // Compile the proto files
    prost_build::Config::new()
        .out_dir(out_dir)
        .compile_protos(&["src/network/proto/game_service.proto"], &["src/proto"])
        .unwrap();
}
