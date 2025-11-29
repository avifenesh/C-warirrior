/**
 * Asset Cache Singleton
 *
 * Provides a global cache for loaded assets to avoid reloading on every mount.
 * Assets are loaded once and reused across the app lifetime.
 */
import { DEFAULT_MANIFEST, loadAssets, type LoadedAssets, type AssetManifest } from './assets';

let globalAssetsPromise: Promise<LoadedAssets> | null = null;
let loadFailed = false;
let loadSucceeded = false;

/**
 * Get globally cached assets. Loads once on first call, returns cached promise thereafter.
 * Safe to call multiple times - will not re-fetch if already loading or loaded.
 */
export function getGlobalAssets(): Promise<LoadedAssets> {
    if (loadFailed) {
        // Don't keep retrying aggressively after a failure
        return Promise.reject(new Error('Asset loading previously failed. Refresh to retry.'));
    }

    if (!globalAssetsPromise) {
        globalAssetsPromise = loadAssets(DEFAULT_MANIFEST)
            .then((assets) => {
                loadSucceeded = true;
                return assets;
            })
            .catch((err) => {
                loadFailed = true;
                globalAssetsPromise = null;
                throw err;
            });
    }

    return globalAssetsPromise;
}

/**
 * Check if assets have been successfully loaded (not just loading).
 */
export function areAssetsLoaded(): boolean {
    return loadSucceeded && !loadFailed;
}

/**
 * Check if asset loading failed.
 */
export function didAssetsFail(): boolean {
    return loadFailed;
}

/**
 * Check if assets are currently loading.
 */
export function areAssetsLoading(): boolean {
    return globalAssetsPromise !== null && !loadSucceeded && !loadFailed;
}

/**
 * Reset the cache (useful for testing or recovery).
 * Forces a fresh load on next getGlobalAssets() call.
 */
export function resetAssetCache(): void {
    globalAssetsPromise = null;
    loadFailed = false;
    loadSucceeded = false;
}

/**
 * Preload level-specific assets (extends global assets with level manifest).
 * Currently uses default manifest; extend when per-level assets are defined.
 *
 * @param _levelId - Level identifier (for future per-level manifests)
 * @param _levelManifest - Optional level-specific manifest to merge with defaults
 */
export async function preloadLevelAssets(
    _levelId: string,
    _levelManifest?: Partial<AssetManifest>
): Promise<LoadedAssets> {
    // For now, just ensure global assets are loaded
    // Future: merge level-specific manifest with defaults and load additional assets
    return getGlobalAssets();
}
