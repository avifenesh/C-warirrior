import type { Backend } from './types';

// Lazily loaded backend storage
let cachedBackend: Backend | null = null;

/**
 * Detect if the application is running in Tauri environment
 * In Tauri 2.0, check for __TAURI_INTERNALS__ instead of __TAURI__
 * @returns {boolean} Whether the app is running in Tauri
 */
export function isTauri(): boolean {
    if (typeof window === 'undefined') return false;
    // Tauri 2.0 uses __TAURI_INTERNALS__
    // Also check for __TAURI__ for backwards compatibility
    return '__TAURI_INTERNALS__' in window || '__TAURI__' in window;
}

/**
 * Get the appropriate backend instance (Tauri or HTTP)
 * Implements singleton pattern to cache backend
 *
 * @returns {Promise<Backend>} Promise resolving to the backend instance
 */
export async function getBackend(): Promise<Backend> {
    // Return cached backend if already loaded
    if (cachedBackend) {
        return cachedBackend;
    }

    const tauriDetected = isTauri();
    console.log('[Backend] Tauri detected:', tauriDetected);
    console.log('[Backend] __TAURI_INTERNALS__:', typeof window !== 'undefined' ? '__TAURI_INTERNALS__' in window : false);
    console.log('[Backend] __TAURI__:', typeof window !== 'undefined' ? '__TAURI__' in window : false);

    // Dynamically import the correct backend based on environment
    if (tauriDetected) {
        console.log('[Backend] Using Tauri backend');
        const { createTauriBackend } = await import('./tauri');
        cachedBackend = createTauriBackend();
    } else {
        console.log('[Backend] Using HTTP backend (no Tauri context)');
        const { createHttpBackend } = await import('./http');
        cachedBackend = createHttpBackend();
    }

    return cachedBackend;
}

// Re-export types for convenience
export * from './types';
