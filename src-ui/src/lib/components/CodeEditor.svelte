<script lang="ts">
    import { createEventDispatcher } from 'svelte';

    interface Props {
        initialCode?: string;
        placeholder?: string;
        submitting?: boolean;
    }

    let { initialCode = '', placeholder = '// Write your C code here', submitting = false }: Props = $props();

    const dispatcher = createEventDispatcher();
    let code = $state(initialCode);

    $effect(() => {
        // keep local state in sync if parent updates initialCode
        code = initialCode;
    });

    // very lightweight syntax highlight: keywords and numbers
    const keywordPattern = /\b(int|float|double|char|void|return|if|else|for|while|do|switch|case|break|continue|struct|typedef|const)\b/g;
    const numberPattern = /\b(0x[\da-fA-F]+|\d+(?:\.\d+)?)\b/g;

    function escapeHtml(str: string) {
        return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
    }

    function highlight(text: string) {
        let out = escapeHtml(text);
        out = out.replace(keywordPattern, '<span class="text-sky-300">$1</span>');
        out = out.replace(numberPattern, '<span class="text-amber-200">$1</span>');
        return out;
    }

    const highlighted = $derived(highlight(code));

    function submit() {
        dispatcher('submit', { code });
    }
</script>

<div class="flex flex-col gap-3">
    <label class="text-xs uppercase tracking-[0.15em] text-slate-400">Code Editor</label>
    <div class="relative rounded-2xl border border-slate-800 bg-slate-950 p-3 shadow-inner shadow-slate-900">
        <pre
            class="pointer-events-none absolute inset-3 m-0 overflow-auto whitespace-pre-wrap font-mono text-sm leading-relaxed text-slate-200"
            aria-hidden="true"
        ><code>{@html highlighted}</code></pre>
        <textarea
            class="relative h-56 w-full resize-none bg-transparent font-mono text-sm text-transparent caret-cyan-300 outline-none"
            spellcheck="false"
            bind:value={code}
            placeholder={placeholder}
        ></textarea>
    </div>
    <div class="flex items-center justify-between text-xs text-slate-400">
        <p>Client-only placeholder. Backend submission happens via parent.</p>
        <button
            class="control-btn"
            on:click={submit}
            disabled={submitting}
        >
            {submitting ? 'Submitting...' : 'Submit Code'}
        </button>
    </div>
</div>
