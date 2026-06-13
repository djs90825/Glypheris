import { useState, useEffect, useRef } from 'react';
import { listen } from '@tauri-apps/api/event';

export interface ExecutionLogEvent {
  session_id: string;
  status: string;
  message: string;
}

export function RuntimeConsole() {
  const [logs, setLogs] = useState<ExecutionLogEvent[]>([]);
  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const unlisten = listen<ExecutionLogEvent>('execution-log', (event) => {
      setLogs((prev) => [...prev, event.payload]);
      
      // Auto-scroll
      setTimeout(() => {
        if (scrollRef.current) {
          scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
        }
      }, 50);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  if (logs.length === 0) return null;

  return (
    <div className="panel" style={{ flex: '1 1 30%', minHeight: 150, marginTop: 8 }}>
      <div className="panel-header">
        <div className="panel-dot" style={{ background: 'var(--neon-green)', boxShadow: '0 0 6px var(--neon-green)' }} />
        <span>Execution Runtime Console</span>
        <button 
            onClick={() => setLogs([])}
            style={{ marginLeft: 'auto', background: 'transparent', border: 'none', color: '#3a6070', fontSize: 10, cursor: 'pointer', fontFamily: 'var(--font-sans)' }}
        >
            ✕ CLEAR
        </button>
      </div>
      <div ref={scrollRef} style={{ flex: 1, overflowY: 'auto', padding: 12, fontFamily: 'var(--font-mono)', fontSize: 11, background: 'var(--void-950)' }}>
        {logs.map((log, i) => {
          let color = '#c8dde8';
          if (log.status === 'ERROR' || log.status === 'CRITICAL') color = 'var(--neon-red)';
          if (log.status === 'SUCCESS' || log.status === 'DONE') color = 'var(--neon-green)';
          if (log.status === 'RUNNING') color = 'var(--neon-cyan)';
          
          return (
            <div key={i} style={{ marginBottom: 4, display: 'flex', gap: 8 }}>
              <span style={{ color: '#3a6070' }}>[{log.status}]</span>
              <span style={{ color }}>{log.message}</span>
            </div>
          );
        })}
      </div>
    </div>
  );
}
