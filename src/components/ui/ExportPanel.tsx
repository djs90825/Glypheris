import { useState } from 'react';
import { useSessionStore, SessionEntry } from '../../store/useSessionStore';
import { revealItemInDir } from '@tauri-apps/plugin-opener';

const SCHEMA_COLOUR: Record<string, string> = {
  GestureCommand:  'var(--neon-cyan)',
  ExecutionPlan:   'var(--neon-violet)',
  InferencePacket: 'var(--neon-amber)',
};

const SCHEMA_ABBR: Record<string, string> = {
  GestureCommand:  'GCO',
  ExecutionPlan:   'EPL',
  InferencePacket: 'INF',
};

function ExportMenu({ entry, onClose }: { entry: SessionEntry; onClose: () => void }) {
  const { exportEntry } = useSessionStore();
  const [exporting, setExporting] = useState<string | null>(null);
  const [lastPath, setLastPath] = useState<string | null>(null);

  const doExport = async (type: 'binary' | 'json' | 'hex_report') => {
    setExporting(type);
    try {
      const path = await exportEntry(entry.id, type);
      setLastPath(path);
      // Auto-reveal the file in the OS explorer
      await revealItemInDir(path).catch(console.error);
    } catch (e) {
      console.error('Export failed:', e);
      alert(`Export failed: ${e}`);
    } finally {
      setExporting(null);
    }
  };

  return (
    <div style={{
      position: 'absolute',
      right: 8,
      top: '100%',
      zIndex: 100,
      background: 'var(--void-800)',
      border: '1px solid rgba(0,245,255,0.2)',
      borderRadius: 6,
      padding: 4,
      minWidth: 160,
      boxShadow: '0 8px 32px rgba(0,0,0,0.6)',
      animation: 'fade-in 0.15s ease-out',
    }}>
      {lastPath && (
        <div style={{
          padding: '6px 10px',
          fontSize: 9,
          color: 'var(--neon-green)',
          fontFamily: 'var(--font-sans)',
          borderBottom: '1px solid rgba(0,245,255,0.1)',
          marginBottom: 4,
          wordBreak: 'break-all',
        }}>
          ✓ Saved
        </div>
      )}
      {([
        { type: 'binary',     label: '⬇ Raw Binary (.bin)',   desc: `${entry.byte_size} bytes` },
        { type: 'json',       label: '⬇ JSON Payload (.json)', desc: 'GBNF validated' },
        { type: 'hex_report', label: '⬇ Hex Report (.txt)',    desc: 'Full annotated dump' },
      ] as const).map(({ type, label, desc }) => (
        <button
          key={type}
          onClick={() => doExport(type)}
          disabled={!!exporting}
          style={{
            display: 'block',
            width: '100%',
            background: 'transparent',
            border: 'none',
            textAlign: 'left',
            padding: '7px 10px',
            cursor: 'pointer',
            borderRadius: 4,
            transition: 'background 0.15s',
          }}
          onMouseEnter={e => (e.currentTarget.style.background = 'rgba(0,245,255,0.08)')}
          onMouseLeave={e => (e.currentTarget.style.background = 'transparent')}
        >
          <div style={{ fontFamily: 'var(--font-sans)', fontSize: 11, color: exporting === type ? 'var(--neon-amber)' : '#c8dde8', fontWeight: 500 }}>
            {exporting === type ? '⏳ Exporting...' : label}
          </div>
          <div style={{ fontFamily: 'var(--font-mono)', fontSize: 9, color: '#3a6070', marginTop: 1 }}>
            {desc}
          </div>
        </button>
      ))}
      <div style={{ borderTop: '1px solid rgba(0,245,255,0.1)', marginTop: 4, paddingTop: 4 }}>
        {entry.schema === 'ExecutionPlan' && (
          <button
            onClick={() => { useSessionStore.getState().executeSession(entry.id); onClose(); }}
            style={{
              display: 'block', width: '100%', background: 'transparent', border: 'none',
              textAlign: 'left', padding: '5px 10px', cursor: 'pointer',
              fontFamily: 'var(--font-sans)', fontSize: 11, color: 'var(--neon-green)', fontWeight: 600,
            }}
          >
            ▶ Execute Plan
          </button>
        )}
        <button
          onClick={() => { useSessionStore.getState().replaySession(entry.id); onClose(); }}
          style={{
            display: 'block', width: '100%', background: 'transparent', border: 'none',
            textAlign: 'left', padding: '5px 10px', cursor: 'pointer',
            fontFamily: 'var(--font-sans)', fontSize: 11, color: '#c8dde8',
          }}
        >
          ↺ Replay Intent
        </button>
        <button
          onClick={() => { useSessionStore.getState().deleteSession(entry.id); onClose(); }}
          style={{
            display: 'block', width: '100%', background: 'transparent', border: 'none',
            textAlign: 'left', padding: '5px 10px', cursor: 'pointer',
            fontFamily: 'var(--font-sans)', fontSize: 11, color: 'var(--neon-red)',
          }}
        >
          ✕ Delete
        </button>
      </div>
    </div>
  );
}

function SessionRow({ entry }: { entry: SessionEntry }) {
  const [menuOpen, setMenuOpen] = useState(false);
  const color = SCHEMA_COLOUR[entry.schema] || 'var(--neon-cyan)';
  const abbr  = SCHEMA_ABBR[entry.schema]  || '???';

  return (
    <div style={{
      display: 'flex',
      alignItems: 'center',
      gap: 10,
      padding: '7px 12px',
      borderBottom: '1px solid rgba(0,245,255,0.05)',
      position: 'relative',
      transition: 'background 0.15s',
    }}
    onMouseEnter={e => (e.currentTarget.style.background = 'rgba(0,245,255,0.03)')}
    onMouseLeave={e => (e.currentTarget.style.background = 'transparent')}
    >
      {/* Schema badge */}
      <div style={{
        flexShrink: 0,
        background: `${color}18`,
        border: `1px solid ${color}40`,
        borderRadius: 4,
        padding: '2px 6px',
        fontFamily: 'var(--font-mono)',
        fontSize: 9,
        fontWeight: 700,
        color,
        letterSpacing: '0.1em',
      }}>
        {abbr}
      </div>

      {/* Timestamp */}
      <div style={{ flexShrink: 0, fontFamily: 'var(--font-mono)', fontSize: 9, color: '#3a6070', minWidth: 76 }}>
        {entry.timestamp.split(' ')[1]}
      </div>

      {/* Intent (truncated) */}
      <div style={{
        flex: 1,
        fontFamily: 'var(--font-sans)',
        fontSize: 11,
        color: '#8ab0c0',
        overflow: 'hidden',
        textOverflow: 'ellipsis',
        whiteSpace: 'nowrap',
      }}>
        {entry.intent}
      </div>

      {/* Byte count */}
      <div style={{ flexShrink: 0, fontFamily: 'var(--font-mono)', fontSize: 10, fontWeight: 700, color: 'var(--neon-green)' }}>
        {entry.byte_size}B
      </div>

      {/* Export button */}
      <div style={{ flexShrink: 0, position: 'relative' }}>
        <button
          onClick={() => setMenuOpen(o => !o)}
          style={{
            background: 'rgba(0,245,255,0.06)',
            border: '1px solid rgba(0,245,255,0.2)',
            borderRadius: 4,
            padding: '3px 8px',
            color: 'var(--neon-cyan)',
            fontFamily: 'var(--font-sans)',
            fontSize: 9,
            fontWeight: 700,
            letterSpacing: '0.08em',
            cursor: 'pointer',
            transition: 'all 0.15s',
          }}
          onMouseEnter={e => (e.currentTarget.style.background = 'rgba(0,245,255,0.12)')}
          onMouseLeave={e => (e.currentTarget.style.background = 'rgba(0,245,255,0.06)')}
        >
          EXPORT ▾
        </button>
        {menuOpen && (
          <ExportMenu entry={entry} onClose={() => setMenuOpen(false)} />
        )}
      </div>
    </div>
  );
}

export function ExportPanel() {
  const { entries, fetchLog, clearLog, isLoading } = useSessionStore();

  return (
    <div className="panel" style={{ flexShrink: 0, maxHeight: 400, minHeight: 250, overflow: 'hidden', display: 'flex', flexDirection: 'column' }}>
      <div className="panel-header" style={{ justifyContent: 'space-between' }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
          <div className="panel-dot" style={{ background: 'var(--neon-violet)', boxShadow: '0 0 6px var(--neon-violet)' }} />
          <span>Session Log</span>
          {entries.length > 0 && (
            <span style={{
              background: 'rgba(191,90,242,0.15)',
              border: '1px solid rgba(191,90,242,0.3)',
              borderRadius: 10,
              padding: '0 6px',
              fontSize: 9,
              color: 'var(--neon-violet)',
              fontFamily: 'var(--font-mono)',
            }}>
              {entries.length}
            </span>
          )}
        </div>
        <div style={{ display: 'flex', gap: 8 }}>
          <button
            onClick={fetchLog}
            disabled={isLoading}
            style={{
              background: 'transparent', border: '1px solid rgba(0,245,255,0.2)', borderRadius: 4,
              padding: '3px 8px', color: '#4aadcc', fontFamily: 'var(--font-sans)', fontSize: 9,
              fontWeight: 700, letterSpacing: '0.08em', cursor: 'pointer',
            }}
          >
            {isLoading ? '...' : '↺ REFRESH'}
          </button>
          {entries.length > 0 && (
            <button
              onClick={clearLog}
              style={{
                background: 'transparent', border: '1px solid rgba(255,59,92,0.2)', borderRadius: 4,
                padding: '3px 8px', color: '#5a3040', fontFamily: 'var(--font-sans)', fontSize: 9,
                fontWeight: 700, letterSpacing: '0.08em', cursor: 'pointer',
              }}
              onMouseEnter={e => {
                (e.currentTarget.style.color = 'var(--neon-red)');
                (e.currentTarget.style.borderColor = 'rgba(255,59,92,0.5)');
              }}
              onMouseLeave={e => {
                (e.currentTarget.style.color = '#5a3040');
                (e.currentTarget.style.borderColor = 'rgba(255,59,92,0.2)');
              }}
            >
              ✕ CLEAR
            </button>
          )}
        </div>
      </div>

      <div style={{ flex: 1, overflowY: 'auto' }}>
        {entries.length === 0 ? (
          <div style={{
            padding: '20px 12px',
            fontFamily: 'var(--font-sans)',
            fontSize: 11,
            color: '#1a3a4a',
            textAlign: 'center',
          }}>
            No compilations yet. Hit ⚡ Compile Intent to begin.
          </div>
        ) : (
          entries.map(entry => <SessionRow key={entry.id} entry={entry} />)
        )}
      </div>
    </div>
  );
}
