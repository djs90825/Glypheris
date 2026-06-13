// Hand-written prost structs — no protoc required.
// Each file mirrors its corresponding .proto schema exactly.

pub mod gesture_command {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/gen/gesture_command.rs"));
}

pub mod execution_plan {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/gen/execution_plan.rs"));
}

pub mod inference_packet {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/gen/inference_packet.rs"));
}

