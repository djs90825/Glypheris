import { useCompilerStore } from '../../store/useCompilerStore';

const PROFILE_ACCENT = {
  GestureCommand:  { color: 'var(--neon-cyan)',   dot: '#00F5FF' },
  ExecutionPlan:   { color: 'var(--neon-violet)', dot: '#BF5AF2' },
  InferencePacket: { color: 'var(--neon-amber)',  dot: '#FFB800' },
};

function HexDisplay({ hex }: { hex: string }) {
  if (!hex) {
    return <div className="hex-output empty">00 00 00 00</div>;
  }
  const bytes = hex.split(' ');
  return (
    <div className="hex-output">
      {bytes.map((byte, i) => (
        <span key={i} className="hex-byte">{byte}</span>
      ))}
    </div>
  );
}

function AsmDisplay({ asm }: { asm: string }) {
  if (!asm) {
    return <div className="asm-output empty">// Awaiting compilation payload...</div>;
  }
  
  // Deterministic syntax-highlighting for strict JSON visualization
  const highlighted = asm
    .replace(/("(?:[^"\\]|\\.)*")\s*:/g, '<span style="color:#4aadcc">$1</span>:')
    .replace(/:\s*("(?:[^"\\]|\\.)*")/g, ': <span style="color:#00FF94">$1</span>')
    .replace(/:\s*(true|false)/g, ': <span style="color:#BF5AF2">$1</span>')
    .replace(/:\s*(-?\d+\.?\d*)/g, ': <span style="color:#FFB800">$1</span>')
    .replace(/^(;.*)/gm, '<span style="color:#2a6080">$1</span>');

  return (
    <div
      className="asm-output"
      dangerouslySetInnerHTML={{ __html: highlighted }}
    />
  );
}

export function TerminalPane() {
  // ATOMIC SELECTORS: Ensuring isolation from external intentInput key-stroke lag.
  const compiledHex = useCompilerStore((state) => state.compiledHex);
  const compiledAsm = useCompilerStore((state) => state.compiledAsm);
  const activeProfile = useCompilerStore((state) => state.activeProfile);
  const isCompiling = useCompilerStore((state) => state.isCompiling);
  const metrics = useCompilerStore((state) => state.metrics);

  const accent = PROFILE_ACCENT[activeProfile];
  const byteCount = compiledHex ? compiledHex.split(' ').length : 0;

  return (
    <div style={{ flex: 1, display: 'flex', flexDirection: 'column', gap: 8, minHeight: 0 }}>

      {/* Structured ASM / JSON Evaluation Panel */}
      <div className="panel" style={{ flex: '1 1 60%', minHeight: 0, position: 'relative' }}>
        <div className="panel-header">
          <div className="panel-dot" style={{ background: accent.dot, boxShadow: `0 0 6px ${accent.dot}` }} />
          <span>Machine Representation</span>
          <span style={{ marginLeft: 8, fontSize: 9, color: '#2a6080', fontFamily: 'var(--font-sans)', fontWeight: 500 }}>
            ASM / JSON
          </span>
          {compiledAsm && (
            <span style={{ marginLeft: 'auto', fontSize: 9, color: accent.color, fontFamily: 'var(--font-sans)' }}>
              ● COMPILED
            </span>
          )}
        </div>

        {isCompiling && (
          <div className="compiling-overlay">
            <div className="compiling-spinner">
              <div className="spinner-ring" />
              <div style={{ fontFamily: 'var(--font-sans)', fontSize: 11, fontWeight: 600, color: 'var(--neon-cyan)', letterSpacing: '0.15em' }}>
                ENGINE ACTIVE
              </div>
              <div style={{ fontSize: 10, color: '#2a6080', letterSpacing: '0.1em' }}>
                Awaiting deterministic structural output...
              </div>
            </div>
          </div>
        )}

        <AsmDisplay asm={compiledAsm} />
      </div>

      {/* Hexadecimal Serialized Binary Display Panel */}
      <div className="panel" style={{ flex: '1 1 40%', minHeight: 0 }}>
        <div className="panel-header">
          <div className="panel-dot" style={{ background: 'var(--neon-cyan)', boxShadow: '0 0 6px var(--neon-cyan)' }} />
          <span>Binary Output</span>
          <span style={{ marginLeft: 8, fontSize: 9, color: '#2a6080', fontFamily: 'var(--font-sans)', fontWeight: 500 }}>
            PROTOBUF / HEX
          </span>
          {byteCount > 0 && (
            <span style={{ marginLeft: 'auto', fontSize: 10, color: 'var(--neon-cyan)', fontFamily: 'var(--font-mono)', fontWeight: 700 }}>
              {byteCount}B
            </span>
          )}
        </div>
        <HexDisplay hex={compiledHex} />

        {/* Dynamic Telemetry Footer */}
        {metrics && (
          <div className="metrics-footer">
            <div className="metric-item">
              <div className="metric-label">TPS</div>
              <div className="metric-value cyan">{metrics.tps.toFixed(2)}</div>
            </div>
            <div className="metric-item">
              <div className="metric-label">TTFT</div>
              <div className="metric-value">{metrics.ttft.toFixed(2)}ms</div>
            </div>
            <div className="metric-item">
              <div className="metric-label">Payload</div>
              <div className="metric-value green">{metrics.byteSize}B</div>
            </div>
            {metrics.byteSize > 0 && (
              <div className="metric-item" style={{ marginLeft: 'auto' }}>
                <div className="metric-label">Compression vs JSON</div>
                <div className="metric-value violet">
                  {compiledAsm
                    ? `${Math.round((1 - metrics.byteSize / (compiledAsm.length - 30)) * 100)}%`
                    : '—'}
                </div>
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}