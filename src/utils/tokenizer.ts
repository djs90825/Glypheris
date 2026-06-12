/**
 * Lightweight client-side token estimator to prevent context window overflow.
 * Approximates 1 token ≈ 4 characters for English text.
 */
export function estimateTokens(text: string): number {
    return Math.ceil(text.length / 4);
}
