<script lang="ts">
    /**
     * AuthGuard Component
     *
     * Protects routes that require authentication.
     * Redirects to login if user is not authenticated.
     * Optionally requires email verification.
     */
    import { goto } from '$app/navigation';
    import { onMount } from 'svelte';
    import { getAuthState, handleOAuthCallback } from '$lib/stores/auth.svelte';

    interface Props {
        /** Require email verification (default: false) */
        requireVerified?: boolean;
        /** Custom redirect URL for unauthenticated users */
        redirectTo?: string;
        /** Show loading state while checking auth */
        children: import('svelte').Snippet;
    }

    let { requireVerified = false, redirectTo = '/login', children }: Props = $props();

    let isChecking = $state(true);
    let isAuthorized = $state(false);

    onMount(() => {
        // Check for OAuth callback first
        handleOAuthCallback();

        checkAuth();
    });

    function checkAuth() {
        const auth = getAuthState();

        // Still loading auth state
        if (auth.isLoading) {
            setTimeout(checkAuth, 100);
            return;
        }

        isChecking = false;

        if (!auth.isAuthenticated) {
            // Not logged in, redirect to login
            const currentPath = window.location.pathname;
            const returnUrl = currentPath !== '/' ? `?return=${encodeURIComponent(currentPath)}` : '';
            goto(`${redirectTo}${returnUrl}`);
            return;
        }

        if (requireVerified && !auth.isEmailVerified) {
            // Logged in but email not verified
            goto('/verify-email-required');
            return;
        }

        // All checks passed
        isAuthorized = true;
    }
</script>

{#if isChecking}
    <div class="auth-guard-loading">
        <div class="loading-content">
            <div class="spinner"></div>
            <p class="loading-text">Verifying access...</p>
        </div>
    </div>
{:else if isAuthorized}
    {@render children()}
{/if}

<style>
    .auth-guard-loading {
        min-height: 100vh;
        display: flex;
        align-items: center;
        justify-content: center;
        background: linear-gradient(135deg, #0f172a 0%, #1e293b 50%, #0f172a 100%);
    }

    .loading-content {
        text-align: center;
    }

    .spinner {
        width: 48px;
        height: 48px;
        border: 4px solid #334155;
        border-top-color: #fbbf24;
        border-radius: 50%;
        animation: spin 1s linear infinite;
        margin: 0 auto 16px;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }

    .loading-text {
        color: #94a3b8;
        font-size: 14px;
    }
</style>

