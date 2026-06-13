import React from 'react';

interface ErrorBoundaryProps {
  children: React.ReactNode;
}
interface ErrorBoundaryState {
  hasError: boolean;
  error: Error | null;
}

export class ErrorBoundary extends React.Component<ErrorBoundaryProps, ErrorBoundaryState> {
  constructor(props: ErrorBoundaryProps) {
    super(props);
    this.state = { hasError: false, error: null };
  }

  static getDerivedStateFromError(error: Error): ErrorBoundaryState {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, info: React.ErrorInfo) {
    console.error('[Glypheris ErrorBoundary]', error, info);
  }

  render() {
    if (this.state.hasError) {
      return (
        <div style={{
          height: '100%',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          background: 'var(--void-950)',
          padding: 32,
        }}>
          <div style={{
            background: 'var(--void-900)',
            border: '1px solid rgba(255,59,92,0.4)',
            borderRadius: 12,
            padding: '40px 48px',
            maxWidth: 560,
            textAlign: 'center',
            boxShadow: '0 0 80px rgba(255,59,92,0.15)',
          }}>
            <div style={{
              fontFamily: 'var(--font-mono)',
              fontSize: 36,
              marginBottom: 16,
              color: 'var(--neon-red)',
            }}>
              0xDEAD
            </div>
            <div style={{
              fontFamily: 'var(--font-sans)',
              fontWeight: 800,
              fontSize: 16,
              letterSpacing: '0.15em',
              textTransform: 'uppercase',
              color: 'var(--neon-red)',
              marginBottom: 12,
            }}>
              Critical Runtime Fault
            </div>
            <div style={{
              fontFamily: 'var(--font-mono)',
              fontSize: 11,
              color: '#3a6070',
              background: 'var(--void-800)',
              border: '1px solid rgba(255,59,92,0.2)',
              borderRadius: 6,
              padding: '12px 16px',
              textAlign: 'left',
              marginBottom: 24,
              lineHeight: 1.8,
            }}>
              {this.state.error?.message || 'Unknown runtime exception'}
            </div>
            <button
              onClick={() => this.setState({ hasError: false, error: null })}
              style={{
                background: 'transparent',
                border: '1px solid rgba(255,59,92,0.4)',
                borderRadius: 6,
                padding: '8px 20px',
                color: 'var(--neon-red)',
                fontFamily: 'var(--font-sans)',
                fontSize: 11,
                fontWeight: 700,
                letterSpacing: '0.1em',
                textTransform: 'uppercase',
                cursor: 'pointer',
              }}
            >
              ↺ Reinitialise Runtime
            </button>
          </div>
        </div>
      );
    }
    return this.props.children;
  }
}
