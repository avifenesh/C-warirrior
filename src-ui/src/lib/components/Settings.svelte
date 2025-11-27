<script lang="ts">
  interface Props {
    isOpen: boolean;
    onClose: () => void;
  }

  let { isOpen, onClose }: Props = $props();

  let volume = $state(50);
  let musicEnabled = $state(true);
  let sfxEnabled = $state(true);
  let theme = $state<'dark' | 'light' | 'crt'>('dark');

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      onClose();
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      onClose();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-overlay" onclick={handleBackdropClick}>
    <div class="modal-panel pixel-border">
      <header class="modal-header">
        <h2>SETTINGS</h2>
        <button class="close-btn" onclick={onClose}>X</button>
      </header>

      <div class="settings-content">
        <section class="setting-group">
          <div class="setting-label">
            <label for="volume">VOLUME</label>
            <span class="value">{volume}%</span>
          </div>
          <input 
            type="range" 
            id="volume" 
            bind:value={volume} 
            min="0" 
            max="100" 
            class="pixel-range"
          />
        </section>

        <section class="setting-group toggle-group">
          <label class="checkbox-label">
            <input type="checkbox" bind:checked={musicEnabled} />
            <span class="checkmark"></span>
            MUSIC
          </label>
          
          <label class="checkbox-label">
            <input type="checkbox" bind:checked={sfxEnabled} />
            <span class="checkmark"></span>
            SFX
          </label>
        </section>

        <section class="setting-group">
          <label for="theme">THEME</label>
          <div class="theme-selector">
            <button 
              class:active={theme === 'dark'} 
              onclick={() => theme = 'dark'}
            >DARK</button>
            <button 
              class:active={theme === 'light'} 
              onclick={() => theme = 'light'}
            >LIGHT</button>
            <button 
              class:active={theme === 'crt'} 
              onclick={() => theme = 'crt'}
            >CRT</button>
          </div>
        </section>

        <section class="setting-group keybinds">
          <h3>KEYBINDS</h3>
          <div class="key-row">
            <span>MOVE</span>
            <span class="keys">W A S D</span>
          </div>
          <div class="key-row">
            <span>INTERACT</span>
            <span class="key">E</span>
          </div>
          <div class="key-row">
            <span>RUN</span>
            <span class="keys">SHIFT</span>
          </div>
        </section>
      </div>
      
      <footer class="modal-footer">
        <button class="action-btn" onclick={onClose}>CLOSE</button>
      </footer>
    </div>
  </div>
{/if}

<style>
  :global(:root) {
    --bg-dark: #1a1a2e;
    --bg-panel: #16213e;
    --accent-cyan: #00fff5;
    --accent-pink: #ff00ff;
    --text-primary: #e0e0e0;
    --text-secondary: #888;
    --success: #00ff00;
    --error: #ff4444;
  }

  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.85);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    backdrop-filter: blur(2px);
  }

  .modal-panel {
    background: var(--bg-panel);
    width: 90%;
    max-width: 500px;
    padding: 20px;
    color: var(--text-primary);
    font-family: 'Press Start 2P', monospace;
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .pixel-border {
    border: 4px solid var(--accent-cyan);
    box-shadow:
      inset 0 0 0 4px var(--bg-dark),
      0 0 20px rgba(0, 255, 245, 0.3);
    image-rendering: pixelated;
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 2px solid var(--bg-dark);
    padding-bottom: 10px;
  }

  h2 {
    color: var(--accent-cyan);
    margin: 0;
    font-size: 1.2rem;
    text-shadow: 2px 2px 0px #000;
  }

  h3 {
    font-size: 0.9rem;
    color: var(--text-secondary);
    margin-bottom: 10px;
  }

  .close-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    font-family: inherit;
    font-size: 1.2rem;
    cursor: pointer;
  }

  .close-btn:hover {
    color: var(--error);
  }

  .settings-content {
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .setting-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .setting-label {
    display: flex;
    justify-content: space-between;
    font-size: 0.8rem;
  }

  .value {
    color: var(--accent-cyan);
  }

  /* Range Slider Styling */
  .pixel-range {
    -webkit-appearance: none;
    appearance: none;
    width: 100%;
    height: 10px;
    background: var(--bg-dark);
    outline: none;
    border: 2px solid var(--text-secondary);
  }

  .pixel-range::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 20px;
    height: 20px;
    background: var(--accent-cyan);
    cursor: pointer;
    border: 2px solid #fff;
  }

  .pixel-range::-moz-range-thumb {
    width: 20px;
    height: 20px;
    background: var(--accent-cyan);
    cursor: pointer;
    border: 2px solid #fff;
    border-radius: 0;
  }

  /* Toggle Group */
  .toggle-group {
    flex-direction: row;
    justify-content: space-around;
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 10px;
    cursor: pointer;
    font-size: 0.8rem;
    user-select: none;
  }

  .checkbox-label input {
    position: absolute;
    opacity: 0;
    cursor: pointer;
    height: 0;
    width: 0;
  }

  .checkmark {
    height: 20px;
    width: 20px;
    background-color: var(--bg-dark);
    border: 2px solid var(--text-secondary);
    display: inline-block;
    position: relative;
  }

  .checkbox-label:hover input ~ .checkmark {
    border-color: var(--accent-cyan);
  }

  .checkbox-label input:checked ~ .checkmark {
    background-color: var(--accent-cyan);
    border-color: var(--accent-cyan);
  }

  .checkbox-label input:checked ~ .checkmark:after {
    content: "";
    position: absolute;
    display: block;
    left: 6px;
    top: 2px;
    width: 4px;
    height: 10px;
    border: solid var(--bg-dark);
    border-width: 0 3px 3px 0;
    transform: rotate(45deg);
  }

  /* Theme Selector */
  .theme-selector {
    display: flex;
    gap: 10px;
  }

  .theme-selector button {
    flex: 1;
    background: var(--bg-dark);
    border: 2px solid var(--text-secondary);
    color: var(--text-secondary);
    padding: 8px;
    font-family: inherit;
    font-size: 0.7rem;
    cursor: pointer;
  }

  .theme-selector button.active {
    background: var(--accent-cyan);
    color: var(--bg-dark);
    border-color: var(--accent-cyan);
    box-shadow: 0 0 10px rgba(0, 255, 245, 0.4);
  }

  /* Keybinds */
  .keybinds {
    background: rgba(0, 0, 0, 0.2);
    padding: 10px;
    border: 1px solid var(--text-secondary);
  }

  .key-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 8px;
    font-size: 0.7rem;
    color: var(--text-primary);
  }

  .keys, .key {
    color: var(--accent-pink);
  }

  .modal-footer {
    display: flex;
    justify-content: center;
    margin-top: 10px;
  }

  .action-btn {
    background: var(--accent-cyan);
    color: var(--bg-dark);
    border: none;
    padding: 12px 24px;
    font-family: inherit;
    font-weight: bold;
    cursor: pointer;
    box-shadow: 4px 4px 0px var(--bg-dark);
    transition: transform 0.1s, box-shadow 0.1s;
  }

  .action-btn:hover {
    transform: translate(-1px, -1px);
    box-shadow: 5px 5px 0px var(--bg-dark);
  }

  .action-btn:active {
    transform: translate(2px, 2px);
    box-shadow: 2px 2px 0px var(--bg-dark);
  }
</style>
