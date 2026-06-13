import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface SocketStatusData {
  running: boolean;
  port: number;
  client_count: number;
}

export function SocketStatus() {
  const [status, setStatus] = useState<SocketStatusData>({
    running: false,
    port: 7432,
    client_count: 0,
  });

  useEffect(() => {
    const poll = async () => {
      try {
        const s = await invoke<SocketStatusData>('get_socket_status');
        setStatus(s);
      } catch {}
    };

    poll();
    const id = setInterval(poll, 2000);
    return () => clearInterval(id);
  }, []);

  const dotColor = status.running
    ? status.client_count > 0
      ? 'var(--neon-green)'
      : 'var(--neon-amber)'
    : 'var(--neon-red)';

  const label = status.running
    ? status.client_count > 0
      ? `${status.client_count} client${status.client_count !== 1 ? 's' : ''}`
      : 'Listening'
    : 'Offline';

  return (
    <div
      title={`IPC Socket · localhost:${status.port} · ${label}`}
      style={{
        display: 'flex',
        alignItems: 'center',
        gap: 6,
        padding: '4px 10px',
        background: 'rgba(0,0,0,0.3)',
        border: `1px solid ${dotColor}30`,
        borderRadius: 20,
        cursor: 'default',
        transition: 'all 0.3s ease',
      }}
    >
      <div style={{
        width: 6,
        height: 6,
        borderRadius: '50%',
        background: dotColor,
        boxShadow: `0 0 6px ${dotColor}`,
        animation: status.client_count > 0 ? 'pulse 1s ease-in-out infinite' : undefined,
      }} />
      <div style={{
        fontFamily: 'var(--font-sans)',
        fontSize: 9,
        fontWeight: 700,
        letterSpacing: '0.1em',
        color: dotColor,
        textTransform: 'uppercase',
      }}>
        IPC :{status.port}
      </div>
      <div style={{
        fontFamily: 'var(--font-mono)',
        fontSize: 9,
        color: '#3a6070',
      }}>
        {label}
      </div>
    </div>
  );
}
