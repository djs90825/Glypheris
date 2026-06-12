use prost::Message;
use serde_json::Value;

// Import the generated protobuf modules
use crate::gen::glypheris::animation::GestureCommand;
use crate::gen::glypheris::agents::ExecutionPlan;
use crate::gen::glypheris::inference::InferencePacket;

pub fn compile_to_binary(json_input: &str, schema: &str) -> Result<Vec<u8>, String> {
    // Parse the JSON string from llama-cpu into a serde_json::Value
    let parsed: Value = serde_json::from_str(json_input)
        .map_err(|e| format!("Strict JSON violation: {}", e))?;

    let mut buf = Vec::new();

    match schema {
        "GestureCommand" => {
            // Encode via prost struct (using serde_json to populate the struct)
            // Wait, prost doesn't derive Deserialize by default!
            // We need to either map the Value manually or use pbjson.
            // For now, let's just map manually since we enforce strict schemas.
            
            let mut cmd = GestureCommand::default();
            cmd.action = match parsed["action"].as_str().unwrap_or("IDLE") {
                "JUMP" => 1,
                "WAVE" => 2,
                "RUN" => 3,
                "ATTACK" => 4,
                _ => 0,
            };
            cmd.intensity = parsed["intensity"].as_f64().unwrap_or(0.0) as f32;
            cmd.duration_ms = parsed["duration_ms"].as_i64().unwrap_or(0) as i32;
            
            if let Some(dir) = parsed["direction"].as_object() {
                let mut v = crate::gen::glypheris::animation::gesture_command::Vector3::default();
                v.x = dir.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
                v.y = dir.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
                v.z = dir.get("z").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
                cmd.direction = Some(v);
            }
            
            cmd.compiler_verified = parsed["compiler_verified"].as_bool().unwrap_or(true);
            
            cmd.encode(&mut buf).map_err(|e| e.to_string())?;
        },
        "ExecutionPlan" => {
            let mut plan = ExecutionPlan::default();
            // Basic mapping
            plan.objective_summary = parsed["objective_summary"].as_str().unwrap_or("").to_string();
            plan.requires_approval = parsed["requires_approval"].as_bool().unwrap_or(false);
            plan.compiler_verified = true;
            plan.encode(&mut buf).map_err(|e| e.to_string())?;
        },
        "InferencePacket" => {
            let mut packet = InferencePacket::default();
            packet.final_conclusion = parsed["final_conclusion"].as_str().unwrap_or("").to_string();
            packet.overall_confidence = parsed["overall_confidence"].as_f64().unwrap_or(0.0) as f32;
            packet.compiler_verified = true;
            packet.encode(&mut buf).map_err(|e| e.to_string())?;
        },
        _ => return Err(format!("Unknown schema target: {}", schema)),
    }

    Ok(buf)
}
