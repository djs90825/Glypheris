import { useEffect } from 'react';
import './App.css';
import { useCompilerStore } from './store/useCompilerStore';
import { EditorPane } from './components/viewport/EditorPane';
import { TerminalPane } from './components/viewport/TerminalPane';
import { ClarificationOverlay } from './components/ui/ClarificationOverlay';
import { ErrorBoundary } from './components/ui/ErrorBoundary';
import { ExportPanel } from './components/ui/ExportPanel';
import { SocketStatus } from './components/ui/SocketStatus';
import { RuntimeConsole } from './components/ui/RuntimeConsole';

const PROFILE_BADGE = {
  GestureCommand:  { label: '3D · ANIM',  color: '#00F5FF', bg: 'rgba(0,245,255,0.1)' },
  ExecutionPlan:   { label: 'AGT · PLAN',  color: '#BF5AF2', bg: 'rgba(191,90,242,0.1)' },
  InferencePacket: { label: 'AI · INFER',  color: '#FFB800', bg: 'rgba(255,184,0,0.1)' },
};

function GlypherisLogo() {
  return (
    <div style={{ display: 'flex', alignItems: 'center', gap: 14 }}>
      {/* Sigil */}
      <div style={{ position: 'relative', width: 36, height: 36 }}>
        <svg viewBox="0 0 36 36" fill="none" xmlns="http://www.w3.org/2000/svg" style={{ width: '100%', height: '100%' }}>
          <polygon points="18,2 34,32 2,32" stroke="#00F5FF" strokeWidth="1.5" fill="none" opacity="0.9"/>
          <polygon points="18,10 28,28 8,28" stroke="#BF5AF2" strokeWidth="1" fill="none" opacity="0.6"/>
          <circle cx="18" cy="20" r="3" fill="#00F5FF" opacity="0.9"/>
          <line x1="18" y1="2" x2="18" y2="17" stroke="#00F5FF" strokeWidth="1" opacity="0.5"/>
        </svg>
        <div style={{
          position: 'absolute',
          inset: -4,
          borderRadius: '50%',
          background: 'radial-gradient(circle, rgba(0,245,255,0.15) 0%, transparent 70%)',
          animation: 'pulse 3s ease-in-out infinite',
          pointerEvents: 'none',
        }} />
      </div>

      {/* Wordmark */}
      <div>
        <div style={{
          fontFamily: 'var(--font-sans)',
          fontWeight: 800,
          fontSize: 22,
          letterSpacing: '0.12em',
          textTransform: 'uppercase',
          background: 'linear-gradient(135deg, #00F5FF 0%, #BF5AF2 100%)',
          WebkitBackgroundClip: 'text',
          WebkitTextFillColor: 'transparent',
          backgroundClip: 'text',
          lineHeight: 1,
        }}>
          Glypheris
        </div>
        <div style={{
          fontFamily: 'var(--font-mono)',
          fontSize: 9,
          letterSpacing: '0.25em',
          color: '#2a6080',
          marginTop: 3,
          textTransform: 'uppercase',
        }}>
          Semantic Intent-to-Machine Bridge · v0.2.0
        </div>
      </div>
    </div>
  );
}

function StatusBar({ profile }: { profile: keyof typeof PROFILE_BADGE }) {
  const badge = PROFILE_BADGE[profile];
  return (
    <div style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
      {/* Active profile badge */}
      <div style={{
        background: badge.bg,
        border: `1px solid ${badge.color}40`,
        borderRadius: 20,
        padding: '4px 12px',
        fontFamily: 'var(--font-sans)',
        fontSize: 9,
        fontWeight: 700,
        letterSpacing: '0.18em',
        color: badge.color,
      }}>
        {badge.label}
      </div>

      {/* System status */}
      <div style={{ display: 'flex', alignItems: 'center', gap: 6, fontFamily: 'var(--font-sans)', fontSize: 9, color: '#2a6080', letterSpacing: '0.1em' }}>
        <div style={{ width: 5, height: 5, borderRadius: '50%', background: 'var(--neon-green)', boxShadow: '0 0 6px var(--neon-green)' }} />
        ENGINE NOMINAL
      </div>

      {/* Version */}
      <div style={{ fontFamily: 'var(--font-mono)', fontSize: 9, color: '#1a3a4a' }}>
        llama.cpp · GBNF · prost
      </div>
    </div>
  );
}

function App() {
  const { isAmbiguousHalt, activeProfile, engineFault, resetCompiler } = useCompilerStore();
  const setIntent = useCompilerStore(s => s.setIntentInput);
  const setActiveProfile = useCompilerStore(s => s.setActiveProfile);
  const triggerCompile = useCompilerStore(s => s.triggerCompilation);

  useEffect(() => {
    const handleReplay = (e: Event) => {
      const customEvent = e as CustomEvent;
      const entry = customEvent.detail;
      if (entry) {
        setActiveProfile(entry.schema as any);
        setIntent(entry.intent);
        setTimeout(() => triggerCompile(), 50); // Delay for state update
      }
    };
    window.addEventListener('glypheris-replay', handleReplay);
    return () => window.removeEventListener('glypheris-replay', handleReplay);
  }, [setActiveProfile, setIntent, triggerCompile]);

  return (
    <ErrorBoundary>
      <div style={{
        height: '100vh',
        display: 'flex',
        flexDirection: 'column',
        background: 'var(--void-950)',
        overflow: 'hidden',
        position: 'relative',
      }}>
        {/* Background grid */}
        <div style={{
          position: 'absolute',
          inset: 0,
          backgroundImage: `
            linear-gradient(rgba(0,245,255,0.025) 1px, transparent 1px),
            linear-gradient(90deg, rgba(0,245,255,0.025) 1px, transparent 1px)
          `,
          backgroundSize: '40px 40px',
          pointerEvents: 'none',
        }} />

        {/* Corner accent — top left */}
        <div style={{
          position: 'absolute',
          top: 0, left: 0,
          width: 200, height: 200,
          background: 'radial-gradient(circle at top left, rgba(0,245,255,0.06) 0%, transparent 70%)',
          pointerEvents: 'none',
        }} />
        {/* Corner accent — bottom right */}
        <div style={{
          position: 'absolute',
          bottom: 0, right: 0,
          width: 300, height: 300,
          background: 'radial-gradient(circle at bottom right, rgba(191,90,242,0.06) 0%, transparent 70%)',
          pointerEvents: 'none',
        }} />

        {/* ── Header ─────────────────────────────────────────────── */}
        <header style={{
          position: 'relative',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
          padding: '14px 20px',
          borderBottom: '1px solid rgba(0,245,255,0.08)',
          background: 'rgba(5,12,20,0.8)',
          backdropFilter: 'blur(10px)',
          flexShrink: 0,
          zIndex: 1,
        }}>
          <GlypherisLogo />
          <div style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
            <StatusBar profile={activeProfile} />
            <SocketStatus />
          </div>
        </header>

        {/* ── Fault Banner (if present) ───────────────────────────── */}
        {engineFault && (
          <div style={{ padding: '0 20px', paddingTop: 12, position: 'relative', zIndex: 1, flexShrink: 0 }}>
            <div className="fault-banner">
              <span style={{ fontSize: 16 }}>⚡</span>
              <span>
                <strong style={{ letterSpacing: '0.1em' }}>Critical Hardware Fault:</strong>{' '}
                {engineFault}
              </span>
              <button
                onClick={resetCompiler}
                style={{
                  marginLeft: 'auto',
                  background: 'transparent',
                  border: 'none',
                  color: 'var(--neon-red)',
                  cursor: 'pointer',
                  fontSize: 14,
                  opacity: 0.6,
                }}
              >
                ✕
              </button>
            </div>
          </div>
        )}

        {/* ── Main workspace ──────────────────────────────────────── */}
        <main style={{
          flex: 1,
          display: 'flex',
          gap: 12,
          padding: '12px 20px 16px',
          overflow: 'hidden',
          position: 'relative',
          zIndex: 1,
          minHeight: 0,
        }}>
          {/* Left: Editor */}
          <div style={{ flex: '0 0 48%', display: 'flex', flexDirection: 'column', minHeight: 0 }}>
            <EditorPane />
          </div>

          {/* Divider */}
          <div style={{
            width: 1,
            background: 'linear-gradient(180deg, transparent, rgba(0,245,255,0.2) 30%, rgba(191,90,242,0.2) 70%, transparent)',
            flexShrink: 0,
            alignSelf: 'stretch',
          }} />

          {/* Right: Terminal output — with ambiguity overlay */}
          <div style={{ flex: 1, display: 'flex', flexDirection: 'column', minHeight: 0, position: 'relative' }}>
            <TerminalPane />
            {isAmbiguousHalt && (
              <ClarificationOverlay />
            )}
          </div>
        </main>

        <div style={{ display: 'flex', padding: '0 20px', gap: 20, marginBottom: 8, zIndex: 1, position: 'relative' }}>
          <RuntimeConsole />
        </div>

        {/* ── Session Log / Export Panel ──────────────────────────── */}
        <div style={{ padding: '0 20px 8px', position: 'relative', zIndex: 1, flexShrink: 0 }}>
          <ExportPanel />
        </div>

        {/* ── Footer ─────────────────────────────────────────────── */}
        <footer style={{
          flexShrink: 0,
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
          padding: '6px 20px',
          borderTop: '1px solid rgba(0,245,255,0.06)',
          background: 'rgba(5,12,20,0.9)',
          position: 'relative',
          zIndex: 1,
        }}>
          <div style={{ fontFamily: 'var(--font-mono)', fontSize: 9, color: '#1a3a4a', letterSpacing: '0.1em' }}>
            GLYPHERIS · SEMANTIC COMPILER FRAMEWORK · PHASE 4
          </div>
          <div style={{ fontFamily: 'var(--font-mono)', fontSize: 9, color: '#1a3a4a' }}>
            ⌘↵ compile · GBNF enforced · protobuf binary
          </div>
        </footer>
      </div>
    </ErrorBoundary>
  );
}

export default App;