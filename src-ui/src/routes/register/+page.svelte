<script lang="ts">
    import { goto } from '$app/navigation';
    import {
        register,
        startGoogleOAuth,
        startGitHubOAuth,
        getAuthState,
    } from '$lib/stores/auth.svelte';
    import { onMount } from 'svelte';

    let email = $state('');
    let password = $state('');
    let confirmPassword = $state('');
    let username = $state('');
    let error = $state<string | null>(null);
    let success = $state<string | null>(null);
    let isSubmitting = $state(false);

    onMount(() => {
        // Redirect if already authenticated
        const auth = getAuthState();
        if (auth.isAuthenticated) {
            goto('/');
        }
    });

    async function handleSubmit(e: Event) {
        e.preventDefault();
        error = null;
        success = null;

        // Validate passwords match
        if (password !== confirmPassword) {
            error = 'Passwords do not match';
            return;
        }

        // Validate password strength
        if (password.length < 8) {
            error = 'Password must be at least 8 characters';
            return;
        }

        if (!/[a-zA-Z]/.test(password) || !/[0-9]/.test(password)) {
            error = 'Password must contain at least one letter and one number';
            return;
        }

        isSubmitting = true;

        const result = await register(email, password, username || undefined);

        isSubmitting = false;

        if (result.success) {
            success = result.message;
        } else {
            error = result.message;
        }
    }

    function handleGoogleLogin() {
        startGoogleOAuth();
    }

    function handleGitHubLogin() {
        startGitHubOAuth();
    }
</script>

<svelte:head>
    <title>Register - Code Warrior</title>
</svelte:head>

<div class="auth-container">
    <div class="auth-card">
        <div class="auth-header">
            <h1 class="auth-title">⚔️ CODE WARRIOR</h1>
            <p class="auth-subtitle">Create your account and begin your journey</p>
        </div>

        {#if success}
            <div class="auth-success">
                <div class="success-icon">✓</div>
                <h2 class="success-title">Check your email!</h2>
                <p class="success-message">{success}</p>
                <a href="/login" class="auth-button primary" style="display: block; text-align: center; text-decoration: none; margin-top: 16px;">
                    Go to Login
                </a>
            </div>
        {:else}
            <form onsubmit={handleSubmit} class="auth-form">
                {#if error}
                    <div class="auth-error">
                        {error}
                    </div>
                {/if}

                <div class="form-group">
                    <label for="username" class="form-label">Username <span class="optional">(optional)</span></label>
                    <input
                        type="text"
                        id="username"
                        bind:value={username}
                        class="form-input"
                        placeholder="CodeWarrior42"
                        autocomplete="username"
                    />
                </div>

                <div class="form-group">
                    <label for="email" class="form-label">Email</label>
                    <input
                        type="email"
                        id="email"
                        bind:value={email}
                        class="form-input"
                        placeholder="warrior@example.com"
                        required
                        autocomplete="email"
                    />
                </div>

                <div class="form-group">
                    <label for="password" class="form-label">Password</label>
                    <input
                        type="password"
                        id="password"
                        bind:value={password}
                        class="form-input"
                        placeholder="••••••••"
                        required
                        autocomplete="new-password"
                        minlength="8"
                    />
                    <span class="form-hint">At least 8 characters with letters and numbers</span>
                </div>

                <div class="form-group">
                    <label for="confirmPassword" class="form-label">Confirm Password</label>
                    <input
                        type="password"
                        id="confirmPassword"
                        bind:value={confirmPassword}
                        class="form-input"
                        placeholder="••••••••"
                        required
                        autocomplete="new-password"
                    />
                </div>

                <button type="submit" class="auth-button primary" disabled={isSubmitting}>
                    {isSubmitting ? 'Creating account...' : 'Create Account'}
                </button>
            </form>

            <div class="auth-divider">
                <span>or sign up with</span>
            </div>

            <div class="oauth-buttons">
                <button onclick={handleGoogleLogin} class="oauth-button google">
                    <svg class="oauth-icon" viewBox="0 0 24 24">
                        <path fill="currentColor" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"/>
                        <path fill="currentColor" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"/>
                        <path fill="currentColor" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"/>
                        <path fill="currentColor" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"/>
                    </svg>
                    Google
                </button>
                <button onclick={handleGitHubLogin} class="oauth-button github">
                    <svg class="oauth-icon" viewBox="0 0 24 24">
                        <path fill="currentColor" d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                    </svg>
                    GitHub
                </button>
            </div>

            <p class="auth-footer-text">
                Already have an account? <a href="/login" class="form-link">Sign in</a>
            </p>
        {/if}
    </div>
</div>

<style>
    .auth-container {
        min-height: 100vh;
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 24px;
        background: linear-gradient(135deg, #0f172a 0%, #1e293b 50%, #0f172a 100%);
    }

    .auth-card {
        width: 100%;
        max-width: 420px;
        background: linear-gradient(180deg, #1e293b 0%, #0f172a 100%);
        border: 2px solid #334155;
        border-radius: 16px;
        padding: 40px 32px;
        box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
    }

    .auth-header {
        text-align: center;
        margin-bottom: 32px;
    }

    .auth-title {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 20px;
        color: #fbbf24;
        text-shadow: 2px 2px 0 #92400e;
        margin-bottom: 8px;
        letter-spacing: 2px;
    }

    .auth-subtitle {
        color: #94a3b8;
        font-size: 14px;
    }

    .auth-form {
        display: flex;
        flex-direction: column;
        gap: 20px;
    }

    .auth-error {
        background: rgba(239, 68, 68, 0.1);
        border: 1px solid #ef4444;
        border-radius: 8px;
        padding: 12px 16px;
        color: #fca5a5;
        font-size: 14px;
    }

    .auth-success {
        text-align: center;
        padding: 24px;
    }

    .success-icon {
        width: 64px;
        height: 64px;
        margin: 0 auto 16px;
        background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%);
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 32px;
        color: white;
    }

    .success-title {
        color: #22c55e;
        font-size: 20px;
        margin-bottom: 8px;
    }

    .success-message {
        color: #94a3b8;
        font-size: 14px;
        line-height: 1.6;
    }

    .form-group {
        display: flex;
        flex-direction: column;
        gap: 6px;
    }

    .form-label {
        color: #e2e8f0;
        font-size: 14px;
        font-weight: 500;
    }

    .optional {
        color: #64748b;
        font-weight: 400;
    }

    .form-input {
        background: #0f172a;
        border: 2px solid #334155;
        border-radius: 8px;
        padding: 12px 16px;
        color: #e2e8f0;
        font-size: 16px;
        transition: border-color 0.2s, box-shadow 0.2s;
    }

    .form-input:focus {
        outline: none;
        border-color: #fbbf24;
        box-shadow: 0 0 0 3px rgba(251, 191, 36, 0.1);
    }

    .form-input::placeholder {
        color: #64748b;
    }

    .form-hint {
        color: #64748b;
        font-size: 12px;
    }

    .form-link {
        color: #fbbf24;
        font-size: 14px;
        text-decoration: none;
        transition: color 0.2s;
    }

    .form-link:hover {
        color: #fcd34d;
        text-decoration: underline;
    }

    .auth-button {
        padding: 14px 24px;
        border-radius: 8px;
        font-size: 16px;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.2s;
        border: none;
    }

    .auth-button.primary {
        background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%);
        color: white;
        box-shadow: 0 4px 14px rgba(34, 197, 94, 0.3);
    }

    .auth-button.primary:hover:not(:disabled) {
        transform: translateY(-2px);
        box-shadow: 0 6px 20px rgba(34, 197, 94, 0.4);
    }

    .auth-button:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .auth-divider {
        display: flex;
        align-items: center;
        gap: 16px;
        margin: 24px 0;
        color: #64748b;
        font-size: 12px;
        text-transform: uppercase;
        letter-spacing: 1px;
    }

    .auth-divider::before,
    .auth-divider::after {
        content: '';
        flex: 1;
        height: 1px;
        background: #334155;
    }

    .oauth-buttons {
        display: flex;
        gap: 12px;
    }

    .oauth-button {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 8px;
        padding: 12px 16px;
        border-radius: 8px;
        font-size: 14px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s;
        border: 2px solid #334155;
        background: #0f172a;
        color: #e2e8f0;
    }

    .oauth-button:hover {
        border-color: #475569;
        background: #1e293b;
    }

    .oauth-icon {
        width: 18px;
        height: 18px;
    }

    .auth-footer-text {
        text-align: center;
        margin-top: 24px;
        color: #94a3b8;
        font-size: 14px;
    }
</style>

