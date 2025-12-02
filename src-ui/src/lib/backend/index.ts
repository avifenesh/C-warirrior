import type { Backend } from './types';

// Lazily loaded backend storage
let cachedBackend: Backend | null = null;

/**
 * Get the appropriate backend instance (WASM first, HTTP fallback)
 *
 * @returns {Promise<Backend>} Promise resolving to the backend instance
 */
export async function getBackend(): Promise<Backend> {
    if (cachedBackend) {
        return cachedBackend;
    }

    try {
        const { createWasmBackend } = await import('./wasm');
        cachedBackend = createWasmBackend();
        return cachedBackend;
    } catch (error) {
        console.warn('[Backend] WASM failed, falling back to HTTP:', error);
        const { createHttpBackend } = await import('./http');
        cachedBackend = createHttpBackend();
        return cachedBackend;
    }
}

// Re-export types for convenience
export * from './types';
