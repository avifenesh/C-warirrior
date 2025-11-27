<script lang="ts">
  interface Achievement {
    id: string;
    title: string;
    description: string;
    icon: string;
    category: 'basics' | 'functions' | 'pointers' | 'advanced';
    unlocked: boolean;
    unlockedAt?: Date;
  }

  interface Props {
    achievements: Achievement[];
    isOpen: boolean;
    onClose: () => void;
  }

  let { achievements, isOpen, onClose }: Props = $props();

  let categories = ['basics', 'functions', 'pointers', 'advanced'] as const;
  let selectedCategory = $state<typeof categories[number]>('basics');

  let filteredAchievements = $derived(
    achievements.filter(a => a.category === selectedCategory)
  );

  let unlockedCount = $derived(achievements.filter(a => a.unlocked).length);

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      onClose();
    }
  }

  function formatDate(date?: Date) {
    if (!date) return '';
    return new Intl.DateTimeFormat('en-US', { month: 'short', day: 'numeric' }).format(date);
  }
</script>

{#if isOpen}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-overlay" onclick={handleBackdropClick}>
    <div class="modal-panel pixel-border">
      <header class="modal-header">
        <h2>ACHIEVEMENTS</h2>
        <div class="trophy-count">
          <span class="trophy-icon">🏆</span>
          <span>{unlockedCount}/{achievements.length}</span>
        </div>
        <button class="close-btn" onclick={onClose}>X</button>
      </header>

      <nav class="category-nav">
        {#each categories as cat}
          <button 
            class:active={selectedCategory === cat}
            onclick={() => selectedCategory = cat}
          >
            {cat.toUpperCase()}
          </button>
        {/each}
      </nav>

      <div class="achievements-grid">
        {#each filteredAchievements as achievement (achievement.id)}
          <div class="achievement-card" class:unlocked={achievement.unlocked}>
            <div class="icon-container">
              <span class="icon">{achievement.icon}</span>
            </div>
            <div class="info">
              <h4>{achievement.title}</h4>
              <p>{achievement.description}</p>
              {#if achievement.unlocked && achievement.unlockedAt}
                <span class="date">Unlocked: {formatDate(achievement.unlockedAt)}</span>
              {/if}
            </div>
          </div>
        {/each}
        
        {#if filteredAchievements.length === 0}
          <div class="empty-state">
            No achievements in this category yet.
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  :global(:root) {
    --bg-dark: #1a1a2e;
    --bg-panel: #16213e;
    --accent-cyan: #00fff5;
    --accent-gold: #ffd700;
    --text-primary: #e0e0e0;
    --text-muted: #666;
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
    max-width: 700px;
    height: 80vh;
    padding: 20px;
    color: var(--text-primary);
    font-family: 'Press Start 2P', monospace;
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .pixel-border {
    border: 4px solid var(--accent-gold);
    box-shadow:
      inset 0 0 0 4px var(--bg-dark),
      0 0 20px rgba(255, 215, 0, 0.3);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 2px solid var(--bg-dark);
    padding-bottom: 10px;
  }

  h2 {
    color: var(--accent-gold);
    margin: 0;
    font-size: 1.2rem;
    text-shadow: 2px 2px 0px #000;
  }

  .trophy-count {
    font-size: 0.8rem;
    color: var(--accent-gold);
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .close-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-family: inherit;
    font-size: 1.2rem;
    cursor: pointer;
  }

  .close-btn:hover {
    color: var(--text-primary);
  }

  .category-nav {
    display: flex;
    gap: 10px;
    overflow-x: auto;
    padding-bottom: 5px;
  }

  .category-nav button {
    background: var(--bg-dark);
    border: 2px solid var(--text-muted);
    color: var(--text-muted);
    padding: 8px 16px;
    font-family: inherit;
    font-size: 0.6rem;
    cursor: pointer;
    white-space: nowrap;
  }

  .category-nav button.active {
    background: var(--accent-gold);
    color: var(--bg-dark);
    border-color: var(--accent-gold);
  }

  .achievements-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 16px;
    overflow-y: auto;
    padding-right: 5px;
    flex: 1;
  }

  /* Scrollbar styling */
  .achievements-grid::-webkit-scrollbar {
    width: 8px;
  }
  .achievements-grid::-webkit-scrollbar-track {
    background: var(--bg-dark);
  }
  .achievements-grid::-webkit-scrollbar-thumb {
    background: var(--text-muted);
  }

  .achievement-card {
    display: flex;
    gap: 12px;
    background: rgba(0, 0, 0, 0.3);
    border: 2px solid var(--text-muted);
    padding: 12px;
    opacity: 0.5;
    transition: all 0.2s;
  }

  .achievement-card.unlocked {
    opacity: 1;
    border-color: var(--accent-gold);
    background: rgba(255, 215, 0, 0.05);
    box-shadow: 0 0 10px rgba(255, 215, 0, 0.1);
  }

  .icon-container {
    width: 50px;
    height: 50px;
    background: var(--bg-dark);
    display: flex;
    align-items: center;
    justify-content: center;
    border: 2px solid currentColor;
    font-size: 1.5rem;
    flex-shrink: 0;
  }

  .achievement-card.unlocked .icon-container {
    border-color: var(--accent-gold);
  }

  .info {
    display: flex;
    flex-direction: column;
    gap: 4px;
    flex: 1;
  }

  h4 {
    margin: 0;
    font-size: 0.8rem;
    color: var(--text-primary);
  }

  .achievement-card.unlocked h4 {
    color: var(--accent-gold);
  }

  p {
    margin: 0;
    font-size: 0.6rem;
    color: var(--text-muted);
    line-height: 1.4;
  }

  .achievement-card.unlocked p {
    color: var(--text-primary);
  }

  .date {
    font-size: 0.5rem;
    color: var(--accent-cyan);
    margin-top: auto;
    text-align: right;
  }

  .empty-state {
    grid-column: 1 / -1;
    text-align: center;
    padding: 40px;
    color: var(--text-muted);
    font-size: 0.8rem;
  }
</style>
