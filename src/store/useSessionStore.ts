import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

export interface SessionEntry {
  id: string;
  timestamp: string;
  schema: 'GestureCommand' | 'ExecutionPlan' | 'InferencePacket';
  intent: string;
  json_payload: string;
  binary_hex: string;
  byte_size: number;
  tps: number;
  ttft: number;
}

interface SessionState {
  entries: SessionEntry[];
  isLoading: boolean;
  fetchLog: () => Promise<void>;
  clearLog: () => Promise<void>;
  exportEntry: (sessionId: string, exportType: 'binary' | 'json' | 'hex_report') => Promise<string>;
  deleteSession: (sessionId: string) => Promise<void>;
  replaySession: (sessionId: string) => void;
  executeSession: (sessionId: string) => Promise<void>;
}

export const useSessionStore = create<SessionState>((set, get) => ({
  entries: [],
  isLoading: false,

  fetchLog: async () => {
    set({ isLoading: true });
    try {
      const entries = await invoke<SessionEntry[]>('get_session_log');
      set({ entries: [...entries].reverse() }); // newest first
    } catch (e) {
      console.error('[SessionStore] fetchLog failed:', e);
    } finally {
      set({ isLoading: false });
    }
  },

  clearLog: async () => {
    await invoke('clear_session_log');
    set({ entries: [] });
  },

  exportEntry: async (sessionId, exportType) => {
    const path = await invoke<string>('export_packet', {
      req: { session_id: sessionId, export_type: exportType },
    });
    return path;
  },

  deleteSession: async (sessionId) => {
    await invoke('delete_session', { sessionId });
    await get().fetchLog();
  },

  replaySession: (sessionId) => {
    const entry = get().entries.find(e => e.id === sessionId);
    if (!entry) return;
    // Set compiler state using custom event or directly via useCompilerStore if possible.
    // For simplicity, we emit a custom event that App.tsx can listen to.
    window.dispatchEvent(new CustomEvent('glypheris-replay', { detail: entry }));
  },

  executeSession: async (sessionId) => {
    try {
      await invoke('execute_session_plan', { sessionId });
    } catch (e) {
      console.error('Execution failed:', e);
    }
  },
}));
