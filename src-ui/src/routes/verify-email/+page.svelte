<script lang="ts">
    import { goto } from '$app/navigation';
    import { page } from '$app/stores';
    import { verifyEmail, getAuthState } from '$lib/stores/auth.svelte';
    import { onMount } from 'svelte';

    let status = $state<'loading' | 'success' | 'error'>('loading');
    let message = $state('Verifying your email...');

    onMount(async () => {
        const token = $page.url.searchParams.get('token');

        if (!token) {
            status = 'error';
            message = 'No verification token provided. Please check your email link.';
            return;
        }

        const result = await verifyEmail(token);

        if (result.success) {
            status = 'success';
            message = 'Email verified successfully! Redirecting...';

            // Redirect to home after short delay
            setTimeout(() => {
                goto('/');
            }, 2000);
        } else {
            status = 'error';
            message = result.message;
        }
    });
</script>

<svelte:head>
    <title>Verify Email - Code Warrior</title>
</svelte:head>

<div class="verify-container">
    <div class="verify-card">
        <div class="verify-header">
            <h1 class="verify-title">⚔️ CODE WARRIOR</h1>
        </div>

        <div class="verify-content">
            {#if status === 'loading'}
                <div class="verify-icon loading">
                    <div class="spinner"></div>
                </div>
                <h2 class="verify-status">Verifying...</h2>
            {:else if status === 'success'}
                <div class="verify-icon success">✓</div>
                <h2 class="verify-status success-text">Email Verified!</h2>
            {:else}
                <div class="verify-icon error">✕</div>
                <h2 class="verify-status error-text">Verification Failed</h2>
            {/if}

            <p class="verify-message">{message}</p>

            {#if status === 'error'}
                <div class="verify-actions">
                    <a href="/login" class="verify-button secondary">Go to Login</a>
                    <a href="/register" class="verify-button primary">Create Account</a>
                </div>
            {/if}
        </div>
    </div>
</div>

<style>
    .verify-container {
        min-height: 100vh;
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 24px;
        background: linear-gradient(135deg, #0f172a 0%, #1e293b 50%, #0f172a 100%);
    }

    .verify-card {
        width: 100%;
        max-width: 420px;
        background: linear-gradient(180deg, #1e293b 0%, #0f172a 100%);
        border: 2px solid #334155;
        border-radius: 16px;
        padding: 40px 32px;
        box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
        text-align: center;
    }

    .verify-header {
        margin-bottom: 32px;
    }

    .verify-title {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 20px;
        color: #fbbf24;
        text-shadow: 2px 2px 0 #92400e;
        letter-spacing: 2px;
    }

    .verify-content {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 16px;
    }

    .verify-icon {
        width: 80px;
        height: 80px;
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 40px;
    }

    .verify-icon.loading {
        background: #1e293b;
        border: 3px solid #334155;
    }

    .verify-icon.success {
        background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%);
        color: white;
    }

    .verify-icon.error {
        background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
        color: white;
    }

    .spinner {
        width: 40px;
        height: 40px;
        border: 4px solid #334155;
        border-top-color: #fbbf24;
        border-radius: 50%;
        animation: spin 1s linear infinite;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }

    .verify-status {
        font-size: 24px;
        color: #e2e8f0;
        margin: 0;
    }

    .success-text {
        color: #22c55e;
    }

    .error-text {
        color: #ef4444;
    }

    .verify-message {
        color: #94a3b8;
        font-size: 14px;
        line-height: 1.6;
        max-width: 300px;
    }

    .verify-actions {
        display: flex;
        gap: 12px;
        margin-top: 16px;
    }

    .verify-button {
        padding: 12px 24px;
        border-radius: 8px;
        font-size: 14px;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.2s;
        text-decoration: none;
        border: none;
    }

    .verify-button.primary {
        background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%);
        color: white;
    }

    .verify-button.secondary {
        background: #334155;
        color: #e2e8f0;
        border: 2px solid #475569;
    }

    .verify-button:hover {
        transform: translateY(-2px);
    }
</style>

