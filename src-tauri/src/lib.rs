use serde::{Deserialize, Serialize};
use std::os::windows::process::CommandExt; // Required for the creation_flags in engine.rs

mod engine;

#[derive(Serialize)]
pub struct CompileResponse {
    status: String,
    binary_hex: String,
    asm: String,
    ambiguity_score: f32,
    tps: f32,
    ttft: f32,
}

#[tauri::command]
async fn compile(intent: String, schema: String) -> Result<CompileResponse, String> {
    println!(
        "Glypheris Intercept - Intent: [{}], Schema: [{}]",
        intent, schema
    );

    let intent_lower = intent.to_lowercase();
    if intent.trim().is_empty()
        || intent_lower.contains("maybe")
        || intent_lower.contains("sort of")
    {
        return Ok(CompileResponse {
            status: "AMBIGUOUS_HALT".to_string(),
            binary_hex: "".to_string(),
            asm: "".to_string(),
            ambiguity_score: 0.89,
            tps: 0.0,
            ttft: 0.0,
        });
    }

    // Resolve structural constraints based on the UI dropdown
    let grammar_path = match schema.as_str() {
        "GestureCommand" => "grammars/gesture_command.gbnf",
        _ => "grammars/gesture_command.gbnf",
    };

    // Ignite the physical local inference engine
    match engine::execute_compilation(&intent, grammar_path) {
        Ok(result) => {
            // Convert the strictly validated JSON string into a raw HEX payload view
            let hex_string: String = result
                .json_payload
                .as_bytes()
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<String>>()
                .join(" ");

            Ok(CompileResponse {
                status: "OK".to_string(),
                binary_hex: hex_string,
                asm: format!("; GBNF VALIDATED JSON PAYLOAD\n{}", result.json_payload),
                ambiguity_score: 0.01,
                tps: result.tps,
                ttft: result.ttft,
            })
        }
        Err(e) => {
            println!("Compilation Failure: {}", e);
            Err(e)
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![compile])
        .run(tauri::generate_context!())
        .expect("Critical failure whilst running Glypheris compiler");
}
