use crate::gen::execution_plan::ExecutionPlan;
use prost::Message;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::Manager;

fn resolve_sandbox_context(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    // UPDATED: Standardised to Tauri v2 path resolution API.
    let root_dir = app.path().app_data_dir().map_err(|e| {
        format!(
            "Failed to resolve deterministic application data storage location: {}",
            e
        )
    })?;

    let sandbox_path = root_dir.join("sandbox");
    if !sandbox_path.exists() {
        fs::create_dir_all(&sandbox_path)
            .map_err(|e| format!("Failed to instantiate execution sandbox workspace: {}", e))?;
    }
    Ok(sandbox_path)
}

#[derive(Serialize, Clone)]
pub struct ExecutionLogEvent {
    pub session_id: String,
    pub status: String,
    pub message: String,
}

fn emit_log(app: &tauri::AppHandle, session_id: &str, status: &str, message: &str) {
    let event = ExecutionLogEvent {
        session_id: session_id.to_string(),
        status: status.to_string(),
        message: message.to_string(),
    };
    let _ = tauri::Emitter::emit(app, "execution-log", event);
}

pub async fn execute_plan(
    app: tauri::AppHandle,
    session_id: String,
    binary: Vec<u8>,
) -> Result<(), String> {
    emit_log(
        &app,
        &session_id,
        "INFO",
        "Initializing Verified Execution Runtime...",
    );

    let sandbox_dir = resolve_sandbox_context(&app)?;

    let plan = match ExecutionPlan::decode(&binary[..]) {
        Ok(p) => p,
        Err(e) => {
            emit_log(
                &app,
                &session_id,
                "ERROR",
                &format!("Failed to decode ExecutionPlan: {}", e),
            );
            return Err(e.to_string());
        }
    };

    emit_log(
        &app,
        &session_id,
        "INFO",
        &format!("Executing Plan: {}", plan.objective),
    );
    emit_log(
        &app,
        &session_id,
        "INFO",
        &format!("Nodes to process: {}", plan.nodes.len()),
    );

    for node in plan.nodes {
        emit_log(
            &app,
            &session_id,
            "RUNNING",
            &format!("Task [{}]: {}", node.task_id, node.tool_name),
        );

        let result = match node.tool_name.as_str() {
            "http_get" => execute_http_get(&node.parameters_json).await,
            "fs_read" => execute_fs_read(&node.parameters_json, &sandbox_dir),
            "fs_write" => execute_fs_write(&node.parameters_json, &sandbox_dir),
            _ => Err(format!("Unknown tool: {}", node.tool_name)),
        };

        match result {
            Ok(out) => {
                emit_log(
                    &app,
                    &session_id,
                    "SUCCESS",
                    &format!(
                        "Task [{}] completed. Output len: {}",
                        node.task_id,
                        out.len()
                    ),
                );
            }
            Err(e) => {
                emit_log(
                    &app,
                    &session_id,
                    "ERROR",
                    &format!("Task [{}] failed: {}", node.task_id, e),
                );
                if plan.max_retries == 0 {
                    emit_log(
                        &app,
                        &session_id,
                        "CRITICAL",
                        "Plan aborted due to task failure.",
                    );
                    return Err(e);
                }
            }
        }
    }

    emit_log(
        &app,
        &session_id,
        "DONE",
        "Plan execution completed successfully.",
    );
    Ok(())
}

// --- Built-in Tools ---

#[derive(Deserialize)]
struct HttpGetParams {
    url: String,
}

async fn execute_http_get(params_json: &str) -> Result<String, String> {
    let params: HttpGetParams = serde_json::from_str(params_json).map_err(|e| e.to_string())?;
    let resp = reqwest::get(&params.url).await.map_err(|e| e.to_string())?;
    let text = resp.text().await.map_err(|e| e.to_string())?;
    Ok(text)
}

#[derive(Deserialize)]
struct FsReadParams {
    filename: String,
}

fn execute_fs_read(params_json: &str, sandbox: &Path) -> Result<String, String> {
    let params: FsReadParams = serde_json::from_str(params_json).map_err(|e| e.to_string())?;
    let path = sandbox.join(params.filename);
    fs::read_to_string(&path).map_err(|e| format!("Failed to read {:?}: {}", path, e))
}

#[derive(Deserialize)]
struct FsWriteParams {
    filename: String,
    content: String,
}

fn execute_fs_write(params_json: &str, sandbox: &Path) -> Result<String, String> {
    let params: FsWriteParams = serde_json::from_str(params_json).map_err(|e| e.to_string())?;
    let path = sandbox.join(params.filename);
    fs::write(&path, params.content).map_err(|e| format!("Failed to write {:?}: {}", path, e))?;
    Ok(format!("Successfully wrote payload to {:?}", path))
}
