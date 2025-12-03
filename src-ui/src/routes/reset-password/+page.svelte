<script lang="ts">
    import { goto } from '$app/navigation';
    import { page } from '$app/stores';
    import { resetPassword } from '$lib/stores/auth.svelte';
    import { onMount } from 'svelte';

    let password = $state('');
    let confirmPassword = $state('');
    let error = $state<string | null>(null);
    let success = $state(false);
    let isSubmitting = $state(false);
    let token = $state<string | null>(null);

    onMount(() => {
        token = $page.url.searchParams.get('token');

        if (!token) {
            error = 'No reset token provided. Please request a new password reset link.';
        }
    });

    async function handleSubmit(e: Event) {
        e.preventDefault();
        error = null;

        if (!token) {
            error = 'No reset token provided';
            return;
        }

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

        const result = await resetPassword(token, password);

        isSubmitting = false;

        if (result.success) {
            success = true;
        } else {
            error = result.message;
        }
    }
</script>

<svelte:head>
    <title>Reset Password - Code Warrior</title>
</svelte:head>

<div class="auth-container">
    <div class="auth-card">
        <div class="auth-header">
            <h1 class="auth-title">⚔️ CODE WARRIOR</h1>
            <p class="auth-subtitle">Create a new password</p>
        </div>

        {#if success}
            <div class="auth-success">
                <div class="success-icon">✓</div>
                <h2 class="success-title">Password Reset!</h2>
                <p class="success-message">
                    Your password has been successfully reset. You can now log in with your new password.
                </p>
                <a href="/login" class="auth-button primary" style="display: block; text-align: center; text-decoration: none; margin-top: 24px;">
                    Go to Login
                </a>
            </div>
        {:else if !token}
            <div class="auth-error-state">
                <div class="error-icon">⚠️</div>
                <h2 class="error-title">Invalid Link</h2>
                <p class="error-message">{error}</p>
                <a href="/forgot-password" class="auth-button primary" style="display: block; text-align: center; text-decoration: none; margin-top: 24px;">
                    Request New Link
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
                    <label for="password" class="form-label">New Password</label>
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
                    <label for="confirmPassword" class="form-label">Confirm New Password</label>
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
                    {isSubmitting ? 'Resetting...' : 'Reset Password'}
                </button>

                <a href="/login" class="back-link">← Back to Login</a>
            </form>
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

    .auth-success, .auth-error-state {
        text-align: center;
        padding: 24px 0;
    }

    .success-icon, .error-icon {
        font-size: 48px;
        margin-bottom: 16px;
    }

    .success-title {
        color: #22c55e;
        font-size: 20px;
        margin-bottom: 8px;
    }

    .error-title {
        color: #ef4444;
        font-size: 20px;
        margin-bottom: 8px;
    }

    .success-message, .error-message {
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

    .back-link {
        display: block;
        text-align: center;
        color: #64748b;
        font-size: 14px;
        text-decoration: none;
        transition: color 0.2s;
    }

    .back-link:hover {
        color: #94a3b8;
    }
</style>

