<script lang="ts">
  interface Props {
    children: import('svelte').Snippet;
    fallback?: import('svelte').Snippet<[Error, () => void]>;
  }

  let { children, fallback }: Props = $props();

  function handleError(e: unknown) {
    console.error("Caught by ErrorBoundary:", e);
  }
</script>

<svelte:boundary onerror={handleError}>
  {@render children()}
  {#snippet failed(error, reset)}
    {@const safeError = error instanceof Error ? error : new Error(String(error))}
    {#if fallback}
      {@render fallback(safeError, reset)}
    {:else}
      <div class="error-container pixel-border">
        <div class="sad-robot">
          [X_X]
        </div>
        <h3>SYSTEM FAILURE</h3>
        <p class="error-message">
          {safeError.message || "Unknown error occurred in the logic core."}
        </p>
        <div class="actions">
          <button class="retry-btn" onclick={reset}>REBOOT SYSTEM</button>
        </div>
      </div>
    {/if}
  {/snippet}
</svelte:boundary>

<style>
  :global(:root) {
    --bg-dark: #1a1a2e;
    --bg-panel: #16213e;
    --accent-cyan: #00fff5;
    --error: #ff4444;
    --text-primary: #e0e0e0;
  }

  .error-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 40px;
    background: var(--bg-panel);
    color: var(--text-primary);
    font-family: 'Press Start 2P', monospace;
    text-align: center;
    gap: 20px;
    max-width: 600px;
    margin: 20px auto;
  }

  .pixel-border {
    border: 4px solid var(--error);
    box-shadow:
      inset 0 0 0 4px var(--bg-dark),
      0 0 20px rgba(255, 68, 68, 0.4);
  }

  .sad-robot {
    font-size: 4rem;
    color: var(--error);
    text-shadow: 4px 4px 0px #000;
    margin-bottom: 10px;
    animation: glitch 2s infinite;
  }

  h3 {
    color: var(--error);
    font-size: 1.5rem;
    margin: 0;
    text-shadow: 2px 2px 0px #000;
  }

  .error-message {
    font-family: monospace;
    font-size: 1rem;
    color: var(--text-primary);
    background: rgba(0, 0, 0, 0.3);
    padding: 10px;
    border: 1px solid var(--error);
    max-width: 100%;
    word-break: break-word;
  }

  .retry-btn {
    background: var(--error);
    color: var(--bg-dark);
    border: none;
    padding: 15px 30px;
    font-family: inherit;
    font-size: 1rem;
    cursor: pointer;
    box-shadow: 4px 4px 0px var(--bg-dark);
    transition: all 0.1s;
  }

  .retry-btn:hover {
    transform: translate(-2px, -2px);
    box-shadow: 6px 6px 0px var(--bg-dark);
    filter: brightness(1.1);
  }

  .retry-btn:active {
    transform: translate(2px, 2px);
    box-shadow: 2px 2px 0px var(--bg-dark);
  }

  @keyframes glitch {
    0% { transform: translate(0) }
    20% { transform: translate(-2px, 2px) }
    40% { transform: translate(-2px, -2px) }
    60% { transform: translate(2px, 2px) }
    80% { transform: translate(2px, -2px) }
    100% { transform: translate(0) }
  }
</style>
