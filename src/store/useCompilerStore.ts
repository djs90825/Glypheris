import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import { estimateTokens } from '../utils/tokenizer';
import { useSessionStore } from './useSessionStore';

interface TelemetryMetrics {
    tps: number;
    ttft: number;
    byteSize: number;
}

interface CompilerState {
    intentInput: string;
    compiledHex: string;
    compiledAsm: string;
    activeProfile: 'InferencePacket' | 'ExecutionPlan' | 'GestureCommand';
    isCompiling: boolean;
    isAmbiguousHalt: boolean;
    entropyScore: number;
    metrics: TelemetryMetrics | null;
    engineFault: string | null;
    
    setIntentInput: (input: string) => void;
    setActiveProfile: (profile: CompilerState['activeProfile']) => void;
    triggerCompilation: () => Promise<void>;
    resolveAmbiguity: (clarifiedInput: string) => void;
    resetCompiler: () => void;
}

export const useCompilerStore = create<CompilerState>((set, get) => ({
    intentInput: "",
    compiledHex: "",
    compiledAsm: "",
    activeProfile: 'GestureCommand',
    isCompiling: false,
    isAmbiguousHalt: false,
    entropyScore: 0,
    metrics: null,
    engineFault: null,

    setIntentInput: (input) => set({ intentInput: input }),
    setActiveProfile: (profile) => set({ activeProfile: profile }),
    
    triggerCompilation: async () => {
        const intent = get().intentInput;
        
        // Context Guard: Enforce buffer limitations cleanly before invoking Rust payload
        const totalTokens = estimateTokens(intent);
        if (totalTokens > 500) {
            set({ engineFault: `Context Buffer Exceeded (${totalTokens} tokens). Please distil input.` });
            return;
        }

        set({ isCompiling: true, isAmbiguousHalt: false, engineFault: null });
        
        try {
            const response = await invoke<{
                status: 'OK' | 'AMBIGUOUS_HALT';
                binary_hex: string;
                asm: string;
                ambiguity_score: number;
                tps: number;
                ttft: number;
                session_id: string;
            }>('compile', {
                intent: get().intentInput,
                schema: get().activeProfile,
            });
            
            if (response.status === 'AMBIGUOUS_HALT') {
                set({ 
                    isCompiling: false, 
                    isAmbiguousHalt: true,
                    entropyScore: response.ambiguity_score 
                });
                return;
            }
            
            set({
                compiledHex: response.binary_hex,
                compiledAsm: response.asm,
                isCompiling: false,
                metrics: {
                    tps: response.tps,
                    ttft: response.ttft,
                    byteSize: response.binary_hex.split(' ').length,
                }
            });

            // Synchronize historical session graph instantly
            await useSessionStore.getState().fetchLog();
            
        } catch (error) {
            console.error("Compilation engine hardware fault intercepted:", error);
            set({ isCompiling: false, engineFault: String(error) }); 
        }
    },
    
    resolveAmbiguity: (clarifiedInput) => {
        set({ intentInput: clarifiedInput, isAmbiguousHalt: false });
        get().triggerCompilation();
    },
    
    resetCompiler: () => {
        set({
            intentInput: "",
            compiledHex: "",
            compiledAsm: "",
            isCompiling: false,
            isAmbiguousHalt: false,
            entropyScore: 0,
            metrics: null,
            engineFault: null
        });
    }
}));