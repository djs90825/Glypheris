use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::fs;
use std::path::PathBuf;
use chrono::Local;
use uuid::Uuid;

pub mod compiler;
pub mod gen;
pub mod api;

// ─── Persistence Helpers ──────────────────────────────────────────────────────

fn history_file() -> Result<PathBuf, String> {
    let home = std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map(PathBuf::from)
        .ok_or("Cannot resolve home directory")?;
    let dir = home.join("Documents").join("Glypheris");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("history.json"))
}

fn load_history() -> Vec<SessionEntry> {
    if let Ok(path) = history_file() {
        if let Ok(data) = fs::read_to_string(&path) {
            if let Ok(entries) = serde_json::from_str(&data) {
                return entries;
            }
        }
    }
    Vec::new()
}

fn save_history(entries: &[SessionEntry]) {
    if let Ok(path) = history_file() {
        if let Ok(data) = serde_json::to_string_pretty(entries) {
            let _ = fs::write(&path, data);
        }
    }
}

// ─── Session Entry ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEntry {
    pub id:           String,
    pub timestamp:    String,
    pub schema:       String,
    pub intent:       String,
    pub json_payload: String,
    pub binary_hex:   String,
    pub byte_size:    usize,
    pub tps:          f32,
    pub ttft:         f32,
}

/// Global session log — stored as Tauri managed state.
pub struct SessionLog(pub Mutex<Vec<SessionEntry>>);

// ─── Compile Response ─────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct CompileResponse {
    status:         String,
    binary_hex:     String,
    asm:            String,
    ambiguity_score: f32,
    tps:            f32,
    ttft:           f32,
    session_id:     String,
}

// ─── Core Compile Command ─────────────────────────────────────────────────────

#[tauri::command]
async fn compile(
    intent: String,
    schema: String,
    session_log: tauri::State<'_, SessionLog>,
    socket_state: tauri::State<'_, api::server::SocketState>,
) -> Result<CompileResponse, String> {
    println!("[Glypheris] Compile — Schema: {}, Intent: {}", schema, intent);

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
            session_id: "".to_string(),
        });
    }

    let grammar_path = match schema.as_str() {
        "GestureCommand"  => "grammars/gesture_command.gbnf",
        "ExecutionPlan"   => "grammars/execution_plan.gbnf",
        "InferencePacket" => "grammars/inference_packet.gbnf",
        _                 => "grammars/gesture_command.gbnf",
    };

    match compiler::engine::execute_compilation(&intent, grammar_path) {
        Ok(result) => {
            let binary = compiler::serializer::compile_to_binary(&result.json_payload, &schema)?;

            let hex_string: String = binary
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ");

            let session_id = Uuid::new_v4().to_string();

            let frame = api::server::frame_packet(&schema, &binary);
            socket_state.broadcast(frame);

            let entry = SessionEntry {
                id:           session_id.clone(),
                timestamp:    Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                schema:       schema.clone(),
                intent:       intent.clone(),
                json_payload: result.json_payload.clone(),
                binary_hex:   hex_string.clone(),
                byte_size:    binary.len(),
                tps:          result.tps,
                ttft:         result.ttft,
            };

            {
                let mut log = session_log.0.lock().unwrap();
                log.push(entry);
                let len = log.len();
                if len > 100 {
                    log.drain(0..len - 100);
                }
                save_history(&log);
            }

            Ok(CompileResponse {
                status:         "OK".to_string(),
                binary_hex:     hex_string,
                asm:            format!("; GBNF VALIDATED JSON PAYLOAD\n{}", result.json_payload),
                ambiguity_score: 0.01,
                tps:            result.tps,
                ttft:           result.ttft,
                session_id,
            })
        }
        Err(e) => {
            println!("[Glypheris] Compilation Failure: {}", e);
            Err(e)
        }
    }
}

// ─── Session & Runtime Commands ───────────────────────────────────────────────

#[tauri::command]
fn get_session_log(session_log: tauri::State<'_, SessionLog>) -> Vec<SessionEntry> {
    session_log.0.lock().unwrap().clone()
}

#[tauri::command]
fn clear_session_log(session_log: tauri::State<'_, SessionLog>) {
    let mut log = session_log.0.lock().unwrap();
    log.clear();
    save_history(&log);
}

#[tauri::command]
fn delete_session(session_id: String, session_log: tauri::State<'_, SessionLog>) {
    let mut log = session_log.0.lock().unwrap();
    log.retain(|e| e.id != session_id);
    save_history(&log);
}

#[tauri::command]
async fn execute_session_plan(
    session_id: String,
    session_log: tauri::State<'_, SessionLog>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let binary = {
        let log = session_log.0.lock().unwrap();
        let entry = log.iter().find(|e| e.id == session_id).ok_or("Session not found")?;
        if entry.schema != "ExecutionPlan" {
            return Err("Only ExecutionPlan sessions can be executed".to_string());
        }
        entry.binary_hex
            .split_whitespace()
            .map(|h| u8::from_str_radix(h, 16).unwrap_or(0))
            .collect::<Vec<u8>>()
    };

    api::runtime::execute_plan(app_handle, session_id, binary).await
}

// ─── Export Commands ──────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct ExportRequest {
    pub session_id:   String,
    pub export_type:  String,
}

#[tauri::command]
fn export_packet(
    req: ExportRequest,
    session_log: tauri::State<'_, SessionLog>,
) -> Result<String, String> {
    let log = session_log.0.lock().unwrap();
    let entry = log
        .iter()
        .find(|e| e.id == req.session_id)
        .ok_or("Session entry not found")?;

    let binary: Vec<u8> = entry.binary_hex
        .split_whitespace()
        .map(|h| u8::from_str_radix(h, 16).unwrap_or(0))
        .collect();

    match req.export_type.as_str() {
        "binary" => api::exporter::export_binary(&binary, &entry.schema, &entry.id),
        "json"   => api::exporter::export_json(&entry.json_payload, &entry.schema, &entry.id),
        "hex_report" => api::exporter::export_hex_report(
            &binary,
            &entry.json_payload,
            &entry.binary_hex,
            &entry.schema,
            &entry.intent,
            &entry.id,
            entry.tps,
        ),
        _ => Err(format!("Unknown export type: {}", req.export_type)),
    }
}

// ─── Socket Status Command ────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct SocketStatus {
    pub running:      bool,
    pub port:         u16,
    pub client_count: usize,
}

#[tauri::command]
fn get_socket_status(
    socket_state: tauri::State<'_, api::server::SocketState>,
) -> SocketStatus {
    SocketStatus {
        running:      true,
        port:         api::server::SOCKET_PORT,
        client_count: socket_state.client_count(),
    }
}

// ─── App Entry Point ──────────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let socket_state = api::server::SocketState::new();
    api::server::start(socket_state.clone());

    let initial_log = load_history();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .manage(SessionLog(Mutex::new(initial_log)))
        .manage(socket_state)
        .invoke_handler(tauri::generate_handler![
            compile,
            get_session_log,
            clear_session_log,
            delete_session,
            execute_session_plan,
            export_packet,
            get_socket_status,
        ])
        .run(tauri::generate_context!())
        .expect("Critical failure whilst running Glypheris compiler");
}

