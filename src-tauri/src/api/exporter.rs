// Phase 5A: Export & Persistence
// Writes compiled payloads to disk in three formats:
//   .bin  — raw Protobuf binary
//   .json — the GBNF-validated JSON payload
//   .txt  — human-readable hex dump report

use std::fs;
use std::path::PathBuf;
use chrono::Local;

fn exports_dir() -> Result<PathBuf, String> {
    let home = dirs_home().ok_or("Cannot resolve home directory")?;
    let dir = home.join("Documents").join("Glypheris").join("exports");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

/// Resolve the user's home directory from environment variables.
fn dirs_home() -> Option<PathBuf> {
    std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map(PathBuf::from)
}

fn timestamp_slug() -> String {
    Local::now().format("%Y%m%d_%H%M%S").to_string()
}

/// Export the raw Protobuf binary to a .bin file.
/// Returns the absolute path of the written file.
pub fn export_binary(binary: &[u8], schema: &str, session_id: &str) -> Result<String, String> {
    let dir = exports_dir()?;
    let filename = format!(
        "{}_{}_{}_{}.bin",
        timestamp_slug(),
        schema.to_lowercase(),
        &session_id[..8],
        binary.len()
    );
    let path = dir.join(&filename);
    fs::write(&path, binary).map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().into_owned())
}

/// Export the validated JSON payload to a .json file.
pub fn export_json(json_payload: &str, schema: &str, session_id: &str) -> Result<String, String> {
    let dir = exports_dir()?;
    let filename = format!(
        "{}_{}_{}_{}.json",
        timestamp_slug(),
        schema.to_lowercase(),
        &session_id[..8],
        json_payload.len()
    );
    let path = dir.join(&filename);
    fs::write(&path, json_payload).map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().into_owned())
}

/// Export a full annotated hex dump report to a .txt file.
pub fn export_hex_report(
    binary: &[u8],
    json_payload: &str,
    hex_string: &str,
    schema: &str,
    intent: &str,
    session_id: &str,
    tps: f32,
) -> Result<String, String> {
    let dir = exports_dir()?;
    let filename = format!(
        "{}_{}_hexreport_{}.txt",
        timestamp_slug(),
        schema.to_lowercase(),
        &session_id[..8]
    );
    let path = dir.join(&filename);

    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Format hex in classic xxd style: 16 bytes per row with ASCII annotation
    let mut hex_dump = String::new();
    for (i, chunk) in binary.chunks(16).enumerate() {
        let offset = format!("{:08X}", i * 16);
        let hex_row: Vec<String> = chunk.iter().map(|b| format!("{:02X}", b)).collect();
        let ascii_row: String = chunk
            .iter()
            .map(|&b| if b >= 0x20 && b < 0x7F { b as char } else { '.' })
            .collect();
        hex_dump.push_str(&format!(
            "{}  {:<47}  |{}|\n",
            offset,
            hex_row.join(" "),
            ascii_row
        ));
    }

    let report = format!(
        r#"═══════════════════════════════════════════════════════════════════════
  GLYPHERIS COMPILER — HEX DUMP REPORT
  Session ID : {}
  Timestamp  : {}
  Schema     : {}
  Byte Count : {} bytes
  TPS        : {:.2}
═══════════════════════════════════════════════════════════════════════

[ ORIGINAL INTENT ]
{}

[ GBNF-VALIDATED JSON PAYLOAD ]
{}

[ PROTOBUF BINARY — HEX INLINE ]
{}

[ PROTOBUF BINARY — xxd DUMP ]
{}
═══════════════════════════════════════════════════════════════════════
"#,
        session_id,
        now,
        schema,
        binary.len(),
        tps,
        intent,
        json_payload,
        hex_string,
        hex_dump
    );

    fs::write(&path, report).map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().into_owned())
}
