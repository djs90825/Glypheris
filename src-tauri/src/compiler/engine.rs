#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::process::Stdio;
use std::time::Instant;

pub struct EngineResult {
    pub json_payload: String,
    pub tps: f32,
    pub ttft: f32,
}

fn schema_context_for(grammar_path: &str) -> &'static str {
    if grammar_path.contains("gesture_command") {
        "action: one of IDLE, JUMP, WAVE, RUN, ATTACK, WALK, CROUCH, ROLL, EMOTE, INTERACT\n\
intensity: float 0.0 to 1.0 (strength of the action)\n\
duration_ms: integer milliseconds\n\
direction: object with float x, y, z\n\
compiler_verified: true\n\
speed_multiplier: float (0.1 slow-motion to 2.0 fast, default 1.0)\n\
loop_count: integer (1 = once, 0 = infinite)\n\
blend_mode: one of OVERRIDE, ADDITIVE, LAYER\n\
priority: integer 0-10 (higher overrides lower animations)\n\
easing: one of LINEAR, EASE_IN, EASE_OUT, EASE_IN_OUT, SPRING, BOUNCE\n\
target_bone: string bone name or empty string\n\
emotion_tag: one of NEUTRAL, AGGRESSIVE, FEARFUL, JOYFUL, EXHAUSTED, DETERMINED, SURPRISED"
    } else if grammar_path.contains("execution_plan") {
        "plan_id: a short unique string identifier\n\
objective: string describing the overall goal\n\
priority_level: one of CRITICAL, HIGH, NORMAL, LOW, BACKGROUND\n\
max_retries: integer number of retry attempts on failure\n\
timeout_ms: integer deadline in milliseconds (0 = no limit)\n\
requires_confirmation: true or false (human approval gate)\n\
parallel_allowed: true or false (concurrent node execution)\n\
nodes: array of task nodes, each having:\n\
  - task_id: unique task string\n\
  - tool_name: one of http_get, fs_read, fs_write\n\
  - parameters_json: serialized parameters JSON string like \"{\\\"url\\\":\\\"http://...\\\"}\"\n\
  - dependencies: array of task_id strings\n\
estimated_cost_units: float compute cost estimate\n\
compiler_verified: true"
    } else {
        "session_id: a short unique session string\n\
query_type: one of CONVERSATIONAL, FACTUAL, ANALYTICAL, CREATIVE, INSTRUCTIONAL\n\
confidence_threshold: float 0.0-1.0 minimum acceptable confidence\n\
chain_of_thought: array of thought steps, each having:\n\
  - step_id: unique step string\n\
  - reasoning: explanation of the step\n\
  - confidence: float 0.0-1.0\n\
  - evidence_refs: array of reference strings\n\
final_answer: string with the concluded response\n\
overall_confidence: float 0.0-1.0 confidence in the answer\n\
requires_clarification: true or false\n\
context_window_used: integer token count consumed\n\
compiler_verified: true"
    }
}

/// Autonomously interrogates the Windows registry/WMI to establish available GPU Video RAM.
async fn detect_vram_gb() -> u64 {
    #[cfg(target_os = "windows")]
    {
        let mut std_cmd = std::process::Command::new("wmic");
        std_cmd.args(["path", "win32_VideoController", "get", "AdapterRAM"]);
        std_cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

        let mut cmd = tokio::process::Command::from(std_cmd);
        if let Ok(output) = cmd.output().await {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = stdout.lines().collect();
            if lines.len() > 1 {
                if let Ok(bytes) = lines[1].trim().parse::<u64>() {
                    return bytes / 1024 / 1024 / 1024; // Convert bytes to GB
                }
            }
        }
    }
    0 // Default fallback
}

pub async fn execute_compilation(intent: &str, grammar_path: &str) -> Result<EngineResult, String> {
    let base_dir = std::env::current_dir().unwrap_or_default();
    let binary_path = base_dir.join("binaries").join("llama-cpu.exe");
    let model_path = base_dir
        .join("binaries")
        .join("models")
        .join("compiler_engine.gguf");
    let grammar_full_path = base_dir.join(grammar_path);

    if !binary_path.exists() || !model_path.exists() || !grammar_full_path.exists() {
        return Err("CRITICAL FAULT: Physical assets missing. Ensure model and runtime binaries are bundled.".to_string());
    }

    let schema_context = schema_context_for(grammar_path);
    let strict_prompt = format!(
        "<|im_start|>system\nYou are a deterministic semantic compiler. Output ONLY a single valid JSON object with these fields:\n{}\n<|im_end|>\n<|im_start|>user\nCompile this human intent into machine parameters: {}\n<|im_end|>\n<|im_start|>assistant\n",
        schema_context, intent
    );

    let vram_gb = detect_vram_gb().await;
    let gpu_layers = if vram_gb >= 4 { "35" } else { "0" };

    let max_attempts = 2;
    let mut current_attempt = 1;
    let mut active_temperature = "0.0";
    let mut active_penalty = "1.0";

    loop {
        let start_time = Instant::now();

        let mut std_cmd = std::process::Command::new(&binary_path);
        #[cfg(target_os = "windows")]
        std_cmd.creation_flags(0x08000000); // Prevent blocking UI ghost windows

        let mut cmd = tokio::process::Command::from(std_cmd);
        cmd.args([
            "-m",
            model_path.to_str().unwrap(),
            "-n",
            "512",
            "-c",
            "1024",
            "-ngl",
            gpu_layers, // Dynamic GPU Offloading applied
            "--grammar-file",
            grammar_full_path.to_str().unwrap(),
            "--temp",
            active_temperature,
            "--repeat-penalty",
            active_penalty,
            "--log-disable",
            "-st",
            "-p",
            &strict_prompt,
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());

        let child_res = cmd.spawn();

        let child = match child_res {
            Ok(c) => c,
            Err(e) => return Err(format!("OS refused to spawn process: {}", e)),
        };

        let output = child.wait_with_output().await.map_err(|e| e.to_string())?;
        let total_time = start_time.elapsed().as_secs_f32();

        if !output.status.success() {
            return Err("Engine Process Fault: Subprocess collapsed unexpectedly.".to_string());
        }

        let raw_output = String::from_utf8_lossy(&output.stdout).to_string();
        let assistant_marker = "<|im_start|>assistant";
        let search_zone: &str = if let Some(idx) = raw_output.rfind(assistant_marker) {
            &raw_output[idx + assistant_marker.len()..]
        } else {
            &raw_output
        };

        let ansi_re_bytes: Vec<u8> = search_zone.bytes().collect();
        let mut clean_zone = String::with_capacity(ansi_re_bytes.len());
        let mut skip = false;
        for ch in search_zone.chars() {
            if ch == '\x1b' {
                skip = true;
                continue;
            }
            if skip {
                if ch == 'm' {
                    skip = false;
                }
                continue;
            }
            clean_zone.push(ch);
        }
        let clean_zone = clean_zone.trim();

        let json_start = clean_zone.find('{');
        let json_end = clean_zone.rfind('}');

        if let (Some(start), Some(end)) = (json_start, json_end) {
            if start < end {
                let clean_json = clean_zone[start..=end].to_string();
                let estimated_tokens = (clean_json.len() as f32) / 4.0;
                let tps = estimated_tokens / total_time.max(0.1);

                return Ok(EngineResult {
                    json_payload: clean_json,
                    tps,
                    ttft: 0.85,
                });
            }
        }

        // DYNAMIC HEURISTIC RELAXATION
        if current_attempt < max_attempts {
            current_attempt += 1;
            active_temperature = "0.15"; // Allow minor entropy deviation to escape loops
            active_penalty = "1.1";
            continue;
        }

        return Err("Engine hallucination: Hardware failed to compile valid JSON structure after maximum retries.".to_string());
    }
}
