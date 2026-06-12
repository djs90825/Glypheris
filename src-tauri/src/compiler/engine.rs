use std::process::{Command, Stdio};
use std::time::Instant;

pub struct EngineResult {
    pub json_payload: String,
    pub tps: f32,
    pub ttft: f32,
}

pub fn execute_compilation(intent: &str, grammar_path: &str) -> Result<EngineResult, String> {
    // Dynamically resolve absolute paths to guarantee execution regardless of terminal launch context
    let base_dir = std::env::current_dir().unwrap_or_default();
    let binary_path = base_dir.join("binaries").join("llama-cpu.exe");
    let model_path = base_dir
        .join("binaries")
        .join("models")
        .join("compiler_engine.gguf");
    let grammar_full_path = base_dir.join(grammar_path);

    // Qwen's strict ChatML format. We leave zero room for the model to hallucinate pleasantries.
    let strict_prompt = format!(
        "<|im_start|>system\nYou are a deterministic semantic compiler. Output ONLY valid JSON conforming to the structural schema.<|im_end|>\n<|im_start|>user\nCompile this human intent into machine parameters: {}\n<|im_end|>\n<|im_start|>assistant\n", 
        intent
    );

    let start_time = Instant::now();

    // Ignite the bare-metal subprocess
    let mut child = Command::new(&binary_path)
        .args([
            "-m",
            model_path.to_str().unwrap(),
            "-n",
            "512", // Max token output
            "-c",
            "2048", // Context window
            "--grammar-file",
            grammar_full_path.to_str().unwrap(),
            "--temp",
            "0.0", // Absolute determinism. Zero creativity.
            "-p",
            &strict_prompt,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        // Hide the console window on Windows when executing
        .creation_flags(0x08000000)
        .spawn()
        .map_err(|e| format!("Failed to ignite LLM core: {}", e))?;

    let output = child.wait_with_output().map_err(|e| e.to_string())?;
    let total_time = start_time.elapsed().as_secs_f32();

    if !output.status.success() {
        let err_str = String::from_utf8_lossy(&output.stderr);
        return Err(format!("LLM Engine Hardware Fault: {}", err_str));
    }

    let raw_output = String::from_utf8_lossy(&output.stdout).to_string();

    // llama.cpp sometimes echoes the prompt. We isolate the absolute JSON boundary.
    let json_start = raw_output.find('{').unwrap_or(0);
    let json_end = raw_output
        .rfind('}')
        .unwrap_or(raw_output.len().saturating_sub(1));

    let clean_json = if json_start <= json_end && raw_output.contains('{') {
        raw_output[json_start..=json_end].to_string()
    } else {
        return Err("Engine hallucinated outside of GBNF constraints.".to_string());
    };

    // Synthesise telemetry metrics based on actual execution time
    let char_count = clean_json.len() as f32;
    // Rough heuristic: 4 chars per token.
    let estimated_tokens = char_count / 4.0;
    let tps = estimated_tokens / total_time.max(0.1);

    Ok(EngineResult {
        json_payload: clean_json,
        tps,
        ttft: 0.85, // Mock TTFT until we implement stderr stream parsing in Phase 3
    })
}
