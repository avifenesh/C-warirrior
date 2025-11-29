<script lang="ts">
  interface Props {
    currentXP: number;
    totalXP: number;
    currentLevel: string;
    completedLevels: string[];
    totalLevels: number;
    nextLevel?: string | null;
  }

  let { currentXP, totalXP, currentLevel, completedLevels, totalLevels, nextLevel = null }: Props = $props();

  // Guard against division by zero
  let progressPercent = $derived(totalLevels > 0 ? (completedLevels.length / totalLevels) * 100 : 0);
  let xpPercent = $derived(totalXP > 0 ? (currentXP / totalXP) * 100 : 0);

  let expanded = $state(false);

  function toggleExpand() {
    expanded = !expanded;
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="progress-tracker pixel-border" class:expanded onclick={toggleExpand}>
  <div class="compact-view">
    <div class="level-badge">
      <span class="label">LVL</span>
      <span class="value">{currentLevel}</span>
    </div>
    
    <div class="bars">
      <div class="bar-container xp-bar-container" title="XP: {currentXP}/{totalXP}">
        <div class="bar-fill xp-fill" style="width: {xpPercent}%"></div>
        <span class="bar-text">XP</span>
      </div>
      
      <div class="bar-container progress-bar-container" title="Progress: {completedLevels.length}/{totalLevels}">
        <div class="bar-fill progress-fill" style="width: {progressPercent}%"></div>
        <span class="bar-text">GAME</span>
      </div>
    </div>
  </div>

  {#if expanded}
    <div class="details-view">
      <div class="detail-row">
        <span>COMPLETED:</span>
        <span class="value">{completedLevels.length} / {totalLevels}</span>
      </div>
      <div class="detail-row">
        <span>XP:</span>
        <span class="value">{currentXP} / {totalXP}</span>
      </div>
      <div class="detail-row">
        <span>NEXT:</span>
        <span class="value">{nextLevel ?? 'COMPLETE'}</span>
      </div>
    </div>
  {/if}
</div>

<style>
  :global(:root) {
    --bg-dark: #1a1a2e;
    --bg-panel: #16213e;
    --accent-cyan: #00fff5;
    --accent-pink: #ff00ff;
    --success: #00ff00;
    --text-primary: #e0e0e0;
  }

  .progress-tracker {
    background: var(--bg-panel);
    color: var(--text-primary);
    font-family: 'Press Start 2P', monospace;
    padding: 8px;
    width: 250px;
    cursor: pointer;
    transition: height 0.2s;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .pixel-border {
    border: 2px solid var(--text-primary);
    box-shadow: 4px 4px 0px var(--bg-dark);
  }

  .progress-tracker:hover {
    border-color: var(--accent-cyan);
  }

  .compact-view {
    display: flex;
    gap: 10px;
    align-items: center;
  }

  .level-badge {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background: var(--bg-dark);
    padding: 5px;
    border: 2px solid var(--accent-pink);
    width: 50px;
    height: 40px;
  }

  .level-badge .label {
    font-size: 0.5rem;
    color: var(--accent-pink);
  }

  .level-badge .value {
    font-size: 0.9rem;
    color: #fff;
  }

  .bars {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .bar-container {
    height: 16px;
    background: var(--bg-dark);
    border: 1px solid var(--text-primary);
    position: relative;
    overflow: hidden;
  }

  .bar-fill {
    height: 100%;
    transition: width 0.5s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .xp-fill {
    background: var(--accent-cyan);
  }

  .progress-fill {
    background: var(--success);
  }

  .bar-text {
    position: absolute;
    top: 50%;
    left: 4px;
    transform: translateY(-50%);
    font-size: 0.5rem;
    z-index: 1;
    text-shadow: 1px 1px 0 #000;
    color: #fff;
  }

  .details-view {
    border-top: 1px dashed var(--text-primary);
    padding-top: 10px;
    display: flex;
    flex-direction: column;
    gap: 6px;
    font-size: 0.6rem;
  }

  .detail-row {
    display: flex;
    justify-content: space-between;
  }

  .detail-row .value {
    color: var(--accent-cyan);
  }
</style>
