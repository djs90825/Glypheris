import { useCompilerStore } from './store/useCompilerStore';

function App() {
  const { 
    intentInput, setIntentInput, activeProfile, setActiveProfile, 
    triggerCompilation, isCompiling, isAmbiguousHalt, entropyScore,
    compiledHex, compiledAsm, metrics, resolveAmbiguity, resetCompiler
  } = useCompilerStore();

  return (
    <div className="flex flex-col h-screen bg-slate-950 text-slate-200 font-mono p-4">
      <header className="flex justify-between items-center border-b border-slate-800 pb-4 mb-4">
        <div>
          <h1 className="text-xl font-bold tracking-widest text-amber-500 uppercase">Glypheris Compiler</h1>
          <p className="text-xs text-slate-500">Semantic Intent-to-Machine Bridge v0.1.0</p>
        </div>
        <div className="flex gap-2">
          <select 
            className="bg-slate-900 border border-slate-700 rounded px-2 py-1 text-xs outline-none focus:border-amber-500"
            value={activeProfile}
            onChange={(e) => setActiveProfile(e.target.value as any)}
          >
            <option value="GestureCommand">GestureCommand (3D/Anim)</option>
            <option value="ExecutionPlan">ExecutionPlan (Agents)</option>
            <option value="InferencePacket">InferencePacket (General AI)</option>
          </select>
        </div>
      </header>

      <main className="flex-1 flex gap-4 overflow-hidden">
        {/* Left Panel: Intent Input */}
        <section className="w-1/2 flex flex-col bg-slate-900 rounded-lg border border-slate-800 overflow-hidden">
          <div className="bg-slate-800 px-3 py-2 text-xs font-bold uppercase tracking-wider text-slate-400 border-b border-slate-700">
            Natural Language Intent
          </div>
          <textarea 
            className="flex-1 bg-transparent p-4 outline-none resize-none text-sm placeholder-slate-600 focus:ring-1 focus:ring-amber-500/50"
            placeholder="Enter human intent here... e.g., 'Make the character jump high'"
            value={intentInput}
            onChange={(e) => setIntentInput(e.target.value)}
            disabled={isCompiling}
          />
          <div className="p-3 bg-slate-800 border-t border-slate-700 flex justify-between items-center">
             <button 
              onClick={resetCompiler}
              className="text-xs px-3 py-1 text-slate-400 hover:text-red-400 transition-colors"
             >
               Clear Buffer
             </button>
             <button 
              onClick={triggerCompilation}
              disabled={isCompiling || !intentInput}
              className="bg-amber-500 hover:bg-amber-400 text-slate-950 px-4 py-2 rounded text-xs font-bold uppercase transition-colors disabled:opacity-50"
             >
              {isCompiling ? 'Compiling...' : 'Compile Intent'}
             </button>
          </div>
        </section>

        {/* Right Panel: Compilation Output */}
        <section className="w-1/2 flex flex-col gap-4">
          {isAmbiguousHalt ? (
            <div className="bg-slate-900 border border-red-500/50 rounded-lg p-4 flex flex-col justify-center items-center h-full text-center relative overflow-hidden">
              <div className="absolute inset-0 bg-red-500/5 animate-pulse"></div>
              <span className="text-red-500 font-bold mb-2 uppercase tracking-widest text-lg relative z-10">Ambiguity Halt</span>
              <p className="text-sm text-slate-300 mb-6 relative z-10">
                High entropy detected: <span className="text-white font-bold">{Math.round(entropyScore * 100)}%</span><br/>
                Intent is too vague for deterministic compilation.
              </p>
              <input 
                type="text" 
                className="bg-slate-950 border border-red-500/50 rounded px-3 py-2 w-3/4 mb-4 text-sm outline-none text-center focus:border-red-400 relative z-10"
                placeholder="Provide precise parameters (e.g., jump height = 2.5m)..."
                onKeyDown={(e) => {
                  if (e.key === 'Enter') resolveAmbiguity(e.currentTarget.value);
                }}
                autoFocus
              />
              <p className="text-xs text-slate-500 relative z-10">Press ENTER to resolve ambiguity and force compilation.</p>
            </div>
          ) : (
            <>
              <div className="flex-1 bg-slate-900 rounded-lg border border-slate-800 flex flex-col overflow-hidden">
                 <div className="bg-slate-800 px-3 py-2 text-xs font-bold uppercase tracking-wider text-slate-400 border-b border-slate-700">
                  Machine Representation (ASM/AST)
                </div>
                <pre className="flex-1 p-4 overflow-auto text-xs text-green-400">
                  {compiledAsm || '// Awaiting compilation payload...'}
                </pre>
              </div>
              
              <div className="h-1/3 bg-slate-900 rounded-lg border border-slate-800 flex flex-col overflow-hidden">
                <div className="bg-slate-800 px-3 py-2 text-xs font-bold uppercase tracking-wider text-slate-400 border-b border-slate-700">
                  Binary Output (HEX)
                </div>
                <pre className="flex-1 p-4 overflow-auto text-xs text-blue-400 break-all font-bold">
                  {compiledHex || '00 00 00 00'}
                </pre>
              </div>
            </>
          )}
        </section>
      </main>

      {/* Metrics Footer */}
      {metrics && !isAmbiguousHalt && (
        <footer className="mt-4 flex gap-6 text-xs text-slate-500 border-t border-slate-800 pt-3">
          <span>TPS: <strong className="text-amber-400">{metrics.tps.toFixed(2)}</strong></span>
          <span>TTFT: <strong className="text-amber-400">{metrics.ttft.toFixed(2)}ms</strong></span>
          <span>Payload Size: <strong className="text-amber-400">{metrics.byteSize} bytes</strong></span>
        </footer>
      )}
    </div>
  );
}

export default App;