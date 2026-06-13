import { useCompilerStore } from '../../store/useCompilerStore';

export function ClarificationOverlay() {
  const { entropyScore, resolveAmbiguity } = useCompilerStore();

  return (
    <div style={{
      position: 'absolute',
      inset: 0,
      background: 'rgba(2,4,8,0.92)',
      backdropFilter: 'blur(8px)',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
      zIndex: 20,
      animation: 'fade-in 0.3s ease-out',
    }}>
      {/* Ambient red glow */}
      <div style={{
        position: 'absolute',
        inset: 0,
        background: 'radial-gradient(circle at center, rgba(255,59,92,0.08) 0%, transparent 70%)',
        animation: 'pulse 2s ease-in-out infinite',
        pointerEvents: 'none',
      }} />

      <div style={{
        position: 'relative',
        background: 'var(--void-900)',
        border: '1px solid rgba(255,59,92,0.4)',
        borderRadius: 12,
        padding: '32px 40px',
        maxWidth: 480,
        width: '100%',
        textAlign: 'center',
        boxShadow: '0 0 60px rgba(255,59,92,0.2)',
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        gap: 20,
      }}>
        {/* Icon */}
        <div style={{
          width: 56,
          height: 56,
          borderRadius: '50%',
          background: 'rgba(255,59,92,0.1)',
          border: '1px solid rgba(255,59,92,0.4)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          fontSize: 24,
        }}>
          ⚠
        </div>

        {/* Title */}
        <div>
          <div style={{
            fontFamily: 'var(--font-sans)',
            fontWeight: 800,
            fontSize: 18,
            letterSpacing: '0.15em',
            textTransform: 'uppercase',
            color: 'var(--neon-red)',
            marginBottom: 6,
          }}>
            Ambiguity Halt
          </div>
          <div style={{ fontSize: 12, color: '#5a8090', lineHeight: 1.6 }}>
            Intent entropy too high for deterministic compilation.
          </div>
        </div>

        {/* Entropy bar */}
        <div style={{ width: '100%' }}>
          <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: 6 }}>
            <span style={{ fontFamily: 'var(--font-sans)', fontSize: 9, fontWeight: 700, letterSpacing: '0.15em', textTransform: 'uppercase', color: '#3a6070' }}>
              Entropy Score
            </span>
            <span style={{ fontFamily: 'var(--font-mono)', fontSize: 12, fontWeight: 700, color: 'var(--neon-red)' }}>
              {Math.round(entropyScore * 100)}%
            </span>
          </div>
          <div style={{ height: 4, background: 'var(--void-700)', borderRadius: 2, overflow: 'hidden' }}>
            <div style={{
              height: '100%',
              width: `${Math.round(entropyScore * 100)}%`,
              background: 'linear-gradient(90deg, var(--neon-amber), var(--neon-red))',
              borderRadius: 2,
              transition: 'width 0.5s ease',
            }} />
          </div>
        </div>

        {/* Input */}
        <input
          type="text"
          autoFocus
          placeholder="Provide precise parameters to resolve..."
          onKeyDown={(e) => {
            if (e.key === 'Enter' && e.currentTarget.value.trim()) {
              resolveAmbiguity(e.currentTarget.value.trim());
            }
          }}
          style={{
            width: '100%',
            background: 'var(--void-800)',
            border: '1px solid rgba(255,59,92,0.35)',
            borderRadius: 6,
            padding: '10px 14px',
            color: '#c8dde8',
            fontFamily: 'var(--font-mono)',
            fontSize: 12,
            outline: 'none',
          }}
          onFocus={e => { e.target.style.borderColor = 'rgba(255,59,92,0.7)'; }}
          onBlur={e => { e.target.style.borderColor = 'rgba(255,59,92,0.35)'; }}
        />

        <div style={{ fontFamily: 'var(--font-sans)', fontSize: 10, color: '#2a4a5a' }}>
          Press <kbd style={{ background: 'var(--void-700)', padding: '2px 6px', borderRadius: 3, color: '#5a8090' }}>Enter</kbd> to force compilation with resolved parameters
        </div>
      </div>
    </div>
  );
}
