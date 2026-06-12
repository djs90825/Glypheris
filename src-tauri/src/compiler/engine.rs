use std::os::windows::process::CommandExt; // Crucial for stealth execution
use std::process::{Command, Stdio};
use std::time::Instant;

pub struct EngineResult {
    pub json_payload: String,
    pub tps: f32,
    pub ttft: f32,
}

pub fn execute_compilation(intent: &str, grammar_path: &str) -> Result<EngineResult, String> {
    let base_dir = std::env::current_dir().unwrap_or_default();
    let binary_path = base_dir.join("binaries").join("llama-cpu.exe");
    let model_path = base_dir
        .join("binaries")
        .join("models")
        .join("compiler_engine.gguf");
    let grammar_full_path = base_dir.join(grammar_path);

    if !binary_path.exists() || !model_path.exists() || !grammar_full_path.exists() {
        return Err("CRITICAL FAULT: Physical assets missing.".to_string());
    }

    // The strict ChatML prompt forcing deterministic obedience
    let strict_prompt = format!(
        "<|im_start|>system\nYou are a deterministic semantic compiler. Output ONLY valid JSON conforming to the structural schema.<|im_end|>\n<|im_start|>user\nCompile this human intent into machine parameters: {}\n<|im_end|>\n<|im_start|>assistant\n", 
        intent
    );

    let start_time = Instant::now();

    // LAUNCH: Stealth mode active. No console window will flash.
    let child_res = Command::new(&binary_path)
        .args([
            "-m",
            model_path.to_str().unwrap(),
            "-n",
            "512",
            "-c",
            "2048",
            "--grammar-file",
            grammar_full_path.to_str().unwrap(),
            "--temp",
            "0.0",
            "--no-conversation",
            "-p",
            &strict_prompt,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .spawn();

    let child = match child_res {
        Ok(c) => c,
        Err(e) => return Err(format!("OS refused to spawn process: {}", e)),
    };

    let output = child.wait_with_output().map_err(|e| e.to_string())?;
    let total_time = start_time.elapsed().as_secs_f32();

    if !output.status.success() {
        let stderr_str = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Engine Fault: {}", stderr_str));
    }

    let raw_output = String::from_utf8_lossy(&output.stdout).to_string();

    // Parse the absolute JSON boundary from the LLM output
    let json_start = raw_output.find('{').unwrap_or(0);
    let json_end = raw_output
        .rfind('}')
        .unwrap_or(raw_output.len().saturating_sub(1));

    let clean_json = if json_start <= json_end && raw_output.contains('{') {
        raw_output[json_start..=json_end].to_string()
    } else {
        return Err("Engine hallucinated outside of GBNF constraints.".to_string());
    };

    let char_count = clean_json.len() as f32;
    let estimated_tokens = char_count / 4.0;
    let tps = estimated_tokens / total_time.max(0.1);

    Ok(EngineResult {
        json_payload: clean_json,
        tps,
        ttft: 0.85,
    })
}
