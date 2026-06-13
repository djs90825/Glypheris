import { useCompilerStore } from '../../store/useCompilerStore';
import { estimateTokens } from '../../utils/tokenizer';

const MAX_TOKENS = 500;

const PROFILE_CONFIG = {
  GestureCommand: {
    label: '3D / Animation',
    accent: 'active-gesture',
    placeholder: 'Describe a character action...\ne.g. "Make the character slowly wave their left arm with a joyful expression, looping forever with spring easing"',
  },
  ExecutionPlan: {
    label: 'Autonomous Agent',
    accent: 'active-agent',
    placeholder: 'Describe an agent task...\ne.g. "Search the web for the latest AI news, summarise it in 3 bullets, then send it to the team Slack channel"',
  },
  InferencePacket: {
    label: 'General AI',
    accent: 'active-inference',
    placeholder: 'Ask anything...\ne.g. "What is the thermodynamic entropy of a black hole, and how does Hawking radiation relate to information loss?"',
  },
} as const;

export function EditorPane() {
  const {
    intentInput, setIntentInput,
    activeProfile, setActiveProfile,
    triggerCompilation, resetCompiler,
    isCompiling,
  } = useCompilerStore();

  const tokenCount = estimateTokens(intentInput);
  const tokenPct = tokenCount / MAX_TOKENS;
  const tokenClass = tokenPct > 0.9 ? 'critical' : tokenPct > 0.7 ? 'warn' : 'safe';
  const config = PROFILE_CONFIG[activeProfile];

  const profiles = Object.entries(PROFILE_CONFIG) as [keyof typeof PROFILE_CONFIG, typeof PROFILE_CONFIG[keyof typeof PROFILE_CONFIG]][];

  return (
    <div className="panel flex-1 flex flex-col" style={{ minHeight: 0 }}>
      {/* Panel header */}
      <div className="panel-header">
        <div className="panel-dot" style={{ background: 'var(--neon-cyan)', boxShadow: '0 0 6px var(--neon-cyan)' }} />
        <span>Natural Language Intent</span>
        <span className={`token-counter ${tokenClass} ml-auto`}>
          {tokenCount} / {MAX_TOKENS} tokens
        </span>
      </div>

      {/* Profile tab switcher */}
      <div style={{ padding: '10px 14px', background: 'var(--void-800)', borderBottom: '1px solid rgba(0,245,255,0.08)' }}>
        <div className="profile-tabs">
          {profiles.map(([key, cfg]) => (
            <button
              key={key}
              className={`profile-tab ${activeProfile === key ? cfg.accent : ''}`}
              onClick={() => setActiveProfile(key)}
            >
              {cfg.label}
            </button>
          ))}
        </div>
      </div>

      {/* Textarea */}
      <textarea
        className="intent-textarea"
        placeholder={config.placeholder}
        value={intentInput}
        onChange={(e) => setIntentInput(e.target.value)}
        disabled={isCompiling}
        onKeyDown={(e) => {
          if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
            e.preventDefault();
            if (!isCompiling && intentInput.trim()) triggerCompilation();
          }
        }}
      />

      {/* Action bar */}
      <div style={{
        padding: '10px 14px',
        background: 'var(--void-800)',
        borderTop: '1px solid rgba(0,245,255,0.08)',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        gap: 12,
      }}>
        <button
          onClick={resetCompiler}
          style={{
            background: 'transparent',
            border: '1px solid rgba(255,59,92,0.25)',
            borderRadius: 5,
            padding: '6px 12px',
            color: '#4a6070',
            fontFamily: 'var(--font-sans)',
            fontSize: 10,
            fontWeight: 600,
            letterSpacing: '0.1em',
            textTransform: 'uppercase',
            cursor: 'pointer',
            transition: 'all 0.2s',
          }}
          onMouseEnter={e => {
            (e.target as HTMLButtonElement).style.color = 'var(--neon-red)';
            (e.target as HTMLButtonElement).style.borderColor = 'var(--neon-red)';
          }}
          onMouseLeave={e => {
            (e.target as HTMLButtonElement).style.color = '#4a6070';
            (e.target as HTMLButtonElement).style.borderColor = 'rgba(255,59,92,0.25)';
          }}
        >
          ✕ Clear Buffer
        </button>

        <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
          <span style={{ fontSize: 10, color: '#2a4a5a', fontFamily: 'var(--font-sans)' }}>
            ⌘↵ to compile
          </span>
          <button
            className={`btn-compile ${isCompiling ? 'compiling' : ''}`}
            onClick={triggerCompilation}
            disabled={isCompiling || !intentInput.trim()}
          >
            {isCompiling ? (
              <span style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                <span style={{ display: 'inline-block', width: 10, height: 10, border: '2px solid', borderTopColor: 'transparent', borderRadius: '50%', animation: 'spin 0.6s linear infinite' }} />
                Compiling...
              </span>
            ) : '⚡ Compile Intent'}
          </button>
        </div>
      </div>
    </div>
  );
}
