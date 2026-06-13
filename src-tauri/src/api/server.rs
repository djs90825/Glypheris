// Phase 5B: IPC Socket Bridge
// Broadcasts compiled Protobuf packets to all connected TCP clients
// on localhost:7432, using a simple framed binary protocol:
//
// ┌─────────────────────────────────────────────────────────────┐
// │ HEADER (12 bytes)                                           │
// │  [0..4]   Magic bytes: 0x47 0x4C 0x50 0x48  ("GLPH")       │
// │  [4..8]   Schema ID (u32 LE): 0=Gesture 1=Exec 2=Infer     │
// │  [8..12]  Payload length in bytes (u32 LE)                  │
// ├─────────────────────────────────────────────────────────────┤
// │ PAYLOAD (N bytes) — raw Protobuf binary                     │
// └─────────────────────────────────────────────────────────────┘

use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;
use tokio::sync::broadcast;

pub const SOCKET_PORT: u16 = 7432;
const MAGIC: &[u8; 4] = b"GLPH";

pub fn schema_id(schema: &str) -> u32 {
    match schema {
        "GestureCommand"  => 0,
        "ExecutionPlan"   => 1,
        "InferencePacket" => 2,
        _                 => 0,
    }
}

/// Frame a binary payload with the GLPH header.
pub fn frame_packet(schema: &str, payload: &[u8]) -> Vec<u8> {
    let mut frame = Vec::with_capacity(12 + payload.len());
    frame.extend_from_slice(MAGIC);
    frame.extend_from_slice(&schema_id(schema).to_le_bytes());
    frame.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    frame.extend_from_slice(payload);
    frame
}

/// Shared state: number of currently connected clients.
#[derive(Clone)]
pub struct SocketState {
    pub client_count: Arc<Mutex<usize>>,
    pub broadcast_tx: broadcast::Sender<Vec<u8>>,
}

impl SocketState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(64);
        SocketState {
            client_count: Arc::new(Mutex::new(0)),
            broadcast_tx: tx,
        }
    }

    pub fn client_count(&self) -> usize {
        *self.client_count.lock().unwrap()
    }

    /// Broadcast a framed packet to all connected clients.
    /// Silently drops if no clients are listening.
    pub fn broadcast(&self, frame: Vec<u8>) {
        let _ = self.broadcast_tx.send(frame);
    }
}

/// Spawn the TCP listener on a Tokio background task.
/// Each accepted client gets its own task that reads from the broadcast channel.
pub fn start(state: SocketState) {
    tauri::async_runtime::spawn(async move {
        let addr = format!("127.0.0.1:{}", SOCKET_PORT);
        let listener = match TcpListener::bind(&addr).await {
            Ok(l) => {
                println!("[Glypheris IPC] Socket server listening on {}", addr);
                l
            }
            Err(e) => {
                eprintln!("[Glypheris IPC] Failed to bind socket: {}", e);
                return;
            }
        };

        loop {
            match listener.accept().await {
                Ok((stream, peer)) => {
                    println!("[Glypheris IPC] Client connected: {}", peer);
                    let state_clone = state.clone();
                    tauri::async_runtime::spawn(async move {
                        handle_client(stream, state_clone).await;
                    });
                }
                Err(e) => {
                    eprintln!("[Glypheris IPC] Accept error: {}", e);
                }
            }
        }
    });
}

async fn handle_client(mut stream: TcpStream, state: SocketState) {
    {
        let mut count = state.client_count.lock().unwrap();
        *count += 1;
    }

    // Send a welcome frame with schema 0xFF (control message)
    let welcome = b"GLPH\xFF\xFF\xFF\xFF\x00\x00\x00\x00";
    let _ = stream.write_all(welcome).await;

    let mut rx = state.broadcast_tx.subscribe();

    loop {
        match rx.recv().await {
            Ok(frame) => {
                if stream.write_all(&frame).await.is_err() {
                    break; // Client disconnected
                }
            }
            Err(broadcast::error::RecvError::Closed) => break,
            Err(broadcast::error::RecvError::Lagged(_)) => continue,
        }
    }

    {
        let mut count = state.client_count.lock().unwrap();
        *count = count.saturating_sub(1);
    }
    println!("[Glypheris IPC] Client disconnected.");
}
