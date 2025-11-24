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

    // Lightweight syntax highlighting using token-based approach
    function escapeHtml(str: string) {
        return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
    }

    function highlight(text: string): string {
        const tokenRules: Array<{ pattern: RegExp; className: string }> = [
            { pattern: /\b(int|float|double|char|void|return|if|else|for|while|do|switch|case|break|continue|struct|typedef|const)\b/g, className: 'hl-keyword' },
            { pattern: /\b(0x[\da-fA-F]+|\d+(?:\.\d+)?)\b/g, className: 'hl-number' },
        ];

        const tokens: Array<{ start: number; end: number; className: string }> = [];
        for (const rule of tokenRules) {
            rule.pattern.lastIndex = 0;
            let match;
            while ((match = rule.pattern.exec(text)) !== null) {
                const start = match.index;
                const end = start + match[0].length;
                const overlaps = tokens.some(t => !(end <= t.start || start >= t.end));
                if (!overlaps) {
                    tokens.push({ start, end, className: rule.className });
                }
            }
        }
        tokens.sort((a, b) => a.start - b.start);

        let result = '';
        let lastEnd = 0;
        for (const token of tokens) {
            result += escapeHtml(text.slice(lastEnd, token.start));
            result += `<span class="${token.className}">${escapeHtml(text.slice(token.start, token.end))}</span>`;
            lastEnd = token.end;
        }
        result += escapeHtml(text.slice(lastEnd));
        return result;
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
