<script lang="ts">
    import { requestPasswordReset } from '$lib/stores/auth.svelte';

    let email = $state('');
    let error = $state<string | null>(null);
    let success = $state(false);
    let isSubmitting = $state(false);

    async function handleSubmit(e: Event) {
        e.preventDefault();
        error = null;
        isSubmitting = true;

        const result = await requestPasswordReset(email);

        isSubmitting = false;

        if (result.success) {
            success = true;
        } else {
            error = result.message;
        }
    }
</script>

<svelte:head>
    <title>Forgot Password - Code Warrior</title>
</svelte:head>

<div class="auth-container">
    <div class="auth-card">
        <div class="auth-header">
            <h1 class="auth-title">⚔️ CODE WARRIOR</h1>
            <p class="auth-subtitle">Reset your password</p>
        </div>

        {#if success}
            <div class="auth-success">
                <div class="success-icon">✉️</div>
                <h2 class="success-title">Check your email</h2>
                <p class="success-message">
                    If an account exists with that email, we've sent a password reset link.
                    The link will expire in 1 hour.
                </p>
                <a href="/login" class="auth-button primary" style="display: block; text-align: center; text-decoration: none; margin-top: 24px;">
                    Back to Login
                </a>
            </div>
        {:else}
            <form onsubmit={handleSubmit} class="auth-form">
                <p class="form-description">
                    Enter your email address and we'll send you a link to reset your password.
                </p>

                {#if error}
                    <div class="auth-error">
                        {error}
                    </div>
                {/if}

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

                <button type="submit" class="auth-button primary" disabled={isSubmitting}>
                    {isSubmitting ? 'Sending...' : 'Send Reset Link'}
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

    .form-description {
        color: #94a3b8;
        font-size: 14px;
        line-height: 1.6;
        text-align: center;
        margin: 0;
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
        padding: 24px 0;
    }

    .success-icon {
        font-size: 48px;
        margin-bottom: 16px;
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

