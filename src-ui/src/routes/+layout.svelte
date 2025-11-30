<script>
    import '../app.css';
</script>

<svelte:head>
    <title>Code Warrior</title>
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <link rel="preconnect" href="https://fonts.googleapis.com" />
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
    <link
        href="https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@400;600;700&family=IBM+Plex+Mono:wght@400;500&display=swap"
        rel="stylesheet"
    />
</svelte:head>

<a href="#main" class="sr-only focus:not-sr-only focus:absolute focus:top-4 focus:left-4 focus:z-50 bg-slate-900 px-3 py-2 rounded-md text-sm">
    Skip to content
</a>

<div class="min-h-screen bg-slate-950 text-slate-50">
    <div class="absolute inset-0 -z-10 bg-[radial-gradient(circle_at_20%_20%,rgba(56,189,248,0.08),transparent_25%),radial-gradient(circle_at_80%_10%,rgba(244,114,182,0.06),transparent_28%),radial-gradient(circle_at_40%_70%,rgba(94,234,212,0.05),transparent_30%)]" aria-hidden="true"></div>

    <div id="main">
        <slot />
    </div>
</div>
