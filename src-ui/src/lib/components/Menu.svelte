<script lang="ts">
    import { createEventDispatcher } from 'svelte';

    const dispatcher = createEventDispatcher();

    function startGame() {
        dispatcher('start');
    }

    function openLevels() {
        dispatcher('levels');
    }

    function continueGame() {
        dispatcher('continue');
    }
</script>

<style lang="postcss">
    @import url('https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap');

    .pixel-menu {
        font-family: 'Press Start 2P', cursive;
        background: linear-gradient(to bottom, #000033, #000066);
        border: 4px solid #FFCC00;
        box-shadow:
            inset -2px -2px 0px #000033,
            inset 2px 2px 0px #000066,
            4px 4px 0px #FFCC00;
        padding: 20px;
        color: #FFFFFF;
        max-width: 400px;
        margin: 0 auto;
    }

    .menu-title {
        color: #FFCC00;
        text-align: center;
        font-size: 24px;
        margin-bottom: 20px;
        text-shadow: 2px 2px 0px #000033;
    }

    .menu-subtitle {
        color: #888888;
        text-align: center;
        font-size: 10px;
        margin-bottom: 15px;
    }

    .pixel-btn {
        font-family: 'Press Start 2P', cursive;
        background-color: #000066;
        color: #FFFFFF;
        border: 2px solid #FFCC00;
        padding: 10px 15px;
        margin: 5px;
        cursor: pointer;
        font-size: 12px;
        transition: all 0.1s;
        box-shadow:
            inset -1px -1px 0px #000033,
            inset 1px 1px 0px #000099;
    }

    .pixel-btn:hover {
        background-color: #0000AA;
        box-shadow:
            inset 1px 1px 0px #000033,
            inset -1px -1px 0px #000099;
    }

    .btn-container {
        display: flex;
        flex-direction: column;
        align-items: center;
    }
</style>

<div class="pixel-menu">
    <div class="menu-subtitle">MAIN MENU</div>
    <h2 class="menu-title">Code Warrior</h2>
    <div class="menu-subtitle">Rust Owns the Game State</div>

    <div class="btn-container">
        <button class="pixel-btn" on:click={continueGame}>Continue</button>
        <button class="pixel-btn" on:click={startGame}>Start Game</button>
        <button class="pixel-btn" on:click={openLevels}>Level Select</button>
    </div>
</div>