fn main() {
    let mut prost_build = prost_build::Config::new();
    prost_build.out_dir("gen");
    prost_build.compile_protos(
        &[
            "proto/gesture_command.proto",
            "proto/execution_plan.proto",
            "proto/inference_packet.proto",
        ],
        &["proto/"],
    ).unwrap_or_else(|e| panic!("Failed to compile protos: {}", e));

    tauri_build::build()
}
