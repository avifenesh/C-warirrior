<script lang="ts">
    interface Props {
        onNewGame?: () => void;
        onContinue?: () => void;
        onSettings?: () => void;
    }

    let { onNewGame, onContinue, onSettings }: Props = $props();
</script>

<div class="main-menu">
    <div class="menu-container">
        <!-- Decorative sword icon -->
        <div class="sword-icon">
            <span>⚔</span>
        </div>

        <!-- Game title -->
        <h1 class="game-title">CODE WARRIOR</h1>
        <p class="game-subtitle">C Mastery</p>

        <!-- Menu buttons -->
        <div class="button-container">
            <button
                class="pixel-button pixel-button-primary"
                onclick={onNewGame}
            >
                NEW QUEST
            </button>

            <button
                class="pixel-button pixel-button-primary"
                onclick={onContinue}
            >
                CONTINUE
            </button>

            <button
                class="pixel-button pixel-button-disabled"
                disabled
                onclick={onSettings}
            >
                SETTINGS
            </button>
        </div>

        <!-- Footer decoration -->
        <div class="footer-text">
            <p>Press any button to begin your journey</p>
        </div>
    </div>
</div>

<style>
    .main-menu {
        position: fixed;
        inset: 0;
        z-index: 50;
        display: flex;
        align-items: center;
        justify-content: center;
        background: linear-gradient(180deg, #1a1a2e 0%, #16213e 100%);
        image-rendering: pixelated;
    }

    .menu-container {
        background: linear-gradient(180deg, #1a1a2e 0%, #16213e 100%);
        border: 4px solid #0f3460;
        border-top-color: #3a506b;
        border-left-color: #3a506b;
        box-shadow:
            inset 0 0 0 2px #0a0a1e,
            8px 8px 0 #0a0a1e;
        padding: 48px 64px;
        min-width: 400px;
        max-width: 500px;
        text-align: center;
    }

    .sword-icon {
        margin-bottom: 24px;
        font-size: 64px;
        line-height: 1;
        filter: drop-shadow(3px 3px 0 #000);
        animation: float 3s ease-in-out infinite;
    }

    @keyframes float {
        0%, 100% {
            transform: translateY(0px);
        }
        50% {
            transform: translateY(-10px);
        }
    }

    .game-title {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 28px;
        color: #fbbf24;
        text-shadow: 3px 3px 0 #92400e;
        margin-bottom: 16px;
        letter-spacing: 4px;
        line-height: 1.4;
    }

    .game-subtitle {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 14px;
        color: #94a3b8;
        text-shadow: 2px 2px 0 #0a0a1e;
        margin-bottom: 48px;
        letter-spacing: 2px;
    }

    .button-container {
        display: flex;
        flex-direction: column;
        gap: 16px;
        margin-bottom: 32px;
    }

    .pixel-button {
        width: 100%;
        padding: 16px 24px;
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 12px;
        text-shadow: 1px 1px 0 #14532d;
        cursor: pointer;
        transition: transform 0.1s, box-shadow 0.1s;
        text-transform: uppercase;
        letter-spacing: 2px;
        border: none;
        outline: none;
    }

    .pixel-button-primary {
        background: linear-gradient(180deg, #166534 0%, #14532d 100%);
        border: 3px solid #22c55e;
        border-bottom-color: #166534;
        border-right-color: #166534;
        box-shadow: 4px 4px 0 #0a0a1e;
        color: #dcfce7;
    }

    .pixel-button-primary:hover {
        transform: translate(2px, 2px);
        box-shadow: 2px 2px 0 #0a0a1e;
    }

    .pixel-button-primary:active {
        transform: translate(4px, 4px);
        box-shadow: none;
    }

    .pixel-button-disabled {
        background: linear-gradient(180deg, #334155 0%, #1e293b 100%);
        border: 3px solid #475569;
        border-bottom-color: #334155;
        border-right-color: #334155;
        box-shadow: 4px 4px 0 #0a0a1e;
        color: #64748b;
        cursor: not-allowed;
        text-shadow: 1px 1px 0 #1e293b;
    }

    .pixel-button-disabled:hover {
        transform: none;
        box-shadow: 4px 4px 0 #0a0a1e;
    }

    .footer-text {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 8px;
        color: #475569;
        text-shadow: 1px 1px 0 #0a0a1e;
        letter-spacing: 1px;
        animation: pulse 2s ease-in-out infinite;
    }

    @keyframes pulse {
        0%, 100% {
            opacity: 0.5;
        }
        50% {
            opacity: 1;
        }
    }
</style>
