import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

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

    setIntentInput: (input) => set({ intentInput: input }),
    setActiveProfile: (profile) => set({ activeProfile: profile }),
    
    triggerCompilation: async () => {
        set({ isCompiling: true, isAmbiguousHalt: false });
        const start = performance.now();
        
        try {
            const response = await invoke<{
                status: 'OK' | 'AMBIGUOUS_HALT';
                binary_hex: string;
                asm: string;
                ambiguity_score: number;
                tps: number;
                ttft: number;
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
                    byteSize: Math.floor(response.binary_hex.length / 2)
                }
            });
            
        } catch (error) {
            console.error("Compilation engine failure:", error);
            set({ isCompiling: false });
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
            metrics: null
        });
    }
}));