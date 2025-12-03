<script lang="ts">
    import { goto } from '$app/navigation';
    import {
        getAuthState,
        logout,
        resendVerification,
        refreshUser,
    } from '$lib/stores/auth.svelte';
    import { onMount } from 'svelte';

    let auth = $state(getAuthState());
    let resendStatus = $state<'idle' | 'sending' | 'sent' | 'error'>('idle');
    let resendMessage = $state('');

    onMount(() => {
        // Redirect if not authenticated
        if (!auth.isAuthenticated) {
            goto('/login');
            return;
        }

        // Refresh user data
        refreshUser();
    });

    // Keep auth state in sync
    $effect(() => {
        auth = getAuthState();
    });

    async function handleLogout() {
        await logout();
        goto('/login');
    }

    async function handleResendVerification() {
        if (!auth.user?.email) return;

        resendStatus = 'sending';
        const result = await resendVerification(auth.user.email);

        if (result.success) {
            resendStatus = 'sent';
            resendMessage = 'Verification email sent!';
        } else {
            resendStatus = 'error';
            resendMessage = result.message;
        }
    }

    function formatDate(dateString: string): string {
        return new Date(dateString).toLocaleDateString('en-US', {
            year: 'numeric',
            month: 'long',
            day: 'numeric',
        });
    }
</script>

<svelte:head>
    <title>Account - Code Warrior</title>
</svelte:head>

<div class="account-container">
    <div class="account-card">
        <div class="account-header">
            <h1 class="account-title">⚔️ Your Account</h1>
            <a href="/" class="back-button">← Back to Game</a>
        </div>

        {#if auth.user}
            <!-- Profile Section -->
            <section class="account-section">
                <h2 class="section-title">Profile</h2>
                <div class="profile-info">
                    <div class="avatar">
                        {auth.user.username?.[0]?.toUpperCase() || auth.user.email[0].toUpperCase()}
                    </div>
                    <div class="profile-details">
                        <p class="profile-name">{auth.user.username || 'Warrior'}</p>
                        <p class="profile-email">{auth.user.email}</p>
                    </div>
                </div>
            </section>

            <!-- Email Verification -->
            {#if !auth.user.email_verified}
                <section class="account-section warning">
                    <div class="warning-icon">⚠️</div>
                    <div class="warning-content">
                        <h3 class="warning-title">Email not verified</h3>
                        <p class="warning-text">
                            Please verify your email to unlock all features.
                        </p>
                        <button
                            onclick={handleResendVerification}
                            class="resend-button"
                            disabled={resendStatus === 'sending'}
                        >
                            {resendStatus === 'sending' ? 'Sending...' : 'Resend Verification'}
                        </button>
                        {#if resendStatus === 'sent' || resendStatus === 'error'}
                            <p class="resend-message {resendStatus}">{resendMessage}</p>
                        {/if}
                    </div>
                </section>
            {/if}

            <!-- Stats Section -->
            <section class="account-section">
                <h2 class="section-title">Stats</h2>
                <div class="stats-grid">
                    <div class="stat-item">
                        <span class="stat-value">{auth.user.total_xp}</span>
                        <span class="stat-label">Total XP</span>
                    </div>
                    <div class="stat-item">
                        <span class="stat-value">{formatDate(auth.user.created_at)}</span>
                        <span class="stat-label">Joined</span>
                    </div>
                </div>
            </section>

            <!-- Account Status -->
            <section class="account-section">
                <h2 class="section-title">Account Status</h2>
                <div class="status-list">
                    <div class="status-item">
                        <span class="status-label">Email</span>
                        <span class="status-value {auth.user.email_verified ? 'verified' : 'unverified'}">
                            {auth.user.email_verified ? '✓ Verified' : '○ Unverified'}
                        </span>
                    </div>
                    <div class="status-item">
                        <span class="status-label">Account ID</span>
                        <span class="status-value mono">{auth.user.id.slice(0, 8)}...</span>
                    </div>
                </div>
            </section>

            <!-- Actions -->
            <section class="account-section">
                <h2 class="section-title">Actions</h2>
                <div class="action-buttons">
                    <a href="/forgot-password" class="action-button secondary">
                        Change Password
                    </a>
                    <button onclick={handleLogout} class="action-button danger">
                        Sign Out
                    </button>
                </div>
            </section>
        {:else}
            <div class="loading">Loading...</div>
        {/if}
    </div>
</div>

<style>
    .account-container {
        min-height: 100vh;
        display: flex;
        align-items: flex-start;
        justify-content: center;
        padding: 48px 24px;
        background: linear-gradient(135deg, #0f172a 0%, #1e293b 50%, #0f172a 100%);
    }

    .account-card {
        width: 100%;
        max-width: 560px;
        background: linear-gradient(180deg, #1e293b 0%, #0f172a 100%);
        border: 2px solid #334155;
        border-radius: 16px;
        padding: 32px;
        box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
    }

    .account-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        margin-bottom: 32px;
        padding-bottom: 16px;
        border-bottom: 1px solid #334155;
    }

    .account-title {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 16px;
        color: #fbbf24;
        text-shadow: 2px 2px 0 #92400e;
        margin: 0;
    }

    .back-button {
        color: #64748b;
        font-size: 14px;
        text-decoration: none;
        transition: color 0.2s;
    }

    .back-button:hover {
        color: #94a3b8;
    }

    .account-section {
        margin-bottom: 32px;
    }

    .account-section.warning {
        background: rgba(245, 158, 11, 0.1);
        border: 1px solid #f59e0b;
        border-radius: 12px;
        padding: 20px;
        display: flex;
        gap: 16px;
    }

    .warning-icon {
        font-size: 24px;
    }

    .warning-content {
        flex: 1;
    }

    .warning-title {
        color: #fbbf24;
        font-size: 16px;
        margin: 0 0 4px;
    }

    .warning-text {
        color: #94a3b8;
        font-size: 14px;
        margin: 0 0 12px;
    }

    .resend-button {
        background: #f59e0b;
        color: #0f172a;
        border: none;
        padding: 8px 16px;
        border-radius: 6px;
        font-size: 14px;
        font-weight: 600;
        cursor: pointer;
        transition: background 0.2s;
    }

    .resend-button:hover:not(:disabled) {
        background: #d97706;
    }

    .resend-button:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .resend-message {
        font-size: 12px;
        margin-top: 8px;
    }

    .resend-message.sent {
        color: #22c55e;
    }

    .resend-message.error {
        color: #ef4444;
    }

    .section-title {
        color: #94a3b8;
        font-size: 12px;
        text-transform: uppercase;
        letter-spacing: 1px;
        margin: 0 0 16px;
    }

    .profile-info {
        display: flex;
        align-items: center;
        gap: 16px;
    }

    .avatar {
        width: 64px;
        height: 64px;
        background: linear-gradient(135deg, #fbbf24 0%, #f59e0b 100%);
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 24px;
        font-weight: 700;
        color: #0f172a;
    }

    .profile-details {
        flex: 1;
    }

    .profile-name {
        color: #e2e8f0;
        font-size: 20px;
        font-weight: 600;
        margin: 0 0 4px;
    }

    .profile-email {
        color: #64748b;
        font-size: 14px;
        margin: 0;
    }

    .stats-grid {
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        gap: 16px;
    }

    .stat-item {
        background: #0f172a;
        border: 1px solid #334155;
        border-radius: 12px;
        padding: 20px;
        text-align: center;
    }

    .stat-value {
        display: block;
        color: #fbbf24;
        font-size: 24px;
        font-weight: 700;
        margin-bottom: 4px;
    }

    .stat-label {
        color: #64748b;
        font-size: 12px;
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .status-list {
        display: flex;
        flex-direction: column;
        gap: 12px;
    }

    .status-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 12px 16px;
        background: #0f172a;
        border: 1px solid #334155;
        border-radius: 8px;
    }

    .status-label {
        color: #94a3b8;
        font-size: 14px;
    }

    .status-value {
        font-size: 14px;
    }

    .status-value.verified {
        color: #22c55e;
    }

    .status-value.unverified {
        color: #f59e0b;
    }

    .status-value.mono {
        font-family: monospace;
        color: #64748b;
    }

    .action-buttons {
        display: flex;
        gap: 12px;
    }

    .action-button {
        flex: 1;
        padding: 12px 20px;
        border-radius: 8px;
        font-size: 14px;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.2s;
        text-align: center;
        text-decoration: none;
        border: none;
    }

    .action-button.secondary {
        background: #334155;
        color: #e2e8f0;
        border: 2px solid #475569;
    }

    .action-button.secondary:hover {
        background: #475569;
    }

    .action-button.danger {
        background: rgba(239, 68, 68, 0.1);
        color: #ef4444;
        border: 2px solid #ef4444;
    }

    .action-button.danger:hover {
        background: rgba(239, 68, 68, 0.2);
    }

    .loading {
        text-align: center;
        color: #64748b;
        padding: 48px;
    }
</style>

