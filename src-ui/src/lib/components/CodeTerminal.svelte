<script lang="ts">
    import { createEventDispatcher, onMount, tick } from 'svelte';

    interface Props {
        initialCode?: string;
        placeholder?: string;
        submitting?: boolean;
        challenge?: string;
        expectedOutput?: string;
        hints?: string[];
        loadingHint?: boolean;
        output?: {
            success?: boolean;
            stdout?: string;
            stderr?: string;
            compile_error?: string;
            message?: string;
        } | null;
        onClose?: () => void;
        onRequestHint?: () => void;
    }

    let {
        initialCode = '',
        placeholder = '// Cast your spell...',
        submitting = false,
        challenge = '',
        expectedOutput = '',
        hints = [],
        loadingHint = false,
        output = null,
        onClose,
        onRequestHint,
    }: Props = $props();

    const dispatcher = createEventDispatcher();
    let code = $state(initialCode);
    let textareaRef: HTMLTextAreaElement | null = null;

    onMount(() => {
        textareaRef?.focus();
    });

    function escapeHtml(str: string) {
        return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
    }

    function highlight(text: string): string {
        const tokenRules: Array<{ pattern: RegExp; className: string }> = [
            { pattern: /\/\/.*$/gm, className: 'hl-comment' },
            { pattern: /"(?:[^"\\]|\\.)*"/g, className: 'hl-string' },
            { pattern: /\b(int|float|double|char|void|return|if|else|for|while|do|switch|case|break|continue|struct|typedef|const|printf|scanf|malloc|free|sizeof|include|define)\b/g, className: 'hl-keyword' },
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
    const isSuccess = $derived.by(() => output?.success === true);
    const isFailure = $derived.by(() => output?.success === false);
    const busy = $derived.by(() => submitting && !output);
    const statusLabel = $derived.by(() => {
        if (busy) return 'Casting...';
        if (isSuccess) return 'Success!';
        if (isFailure) return 'Failed';
        return 'Ready';
    });

    function submit() {
        dispatcher('submit', { code });
    }

    function handleClose() {
        if (onClose) {
            onClose();
        }
    }

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === 'Escape' && onClose) {
            event.preventDefault();
            handleClose();
        }
        if ((event.ctrlKey || event.metaKey) && event.key === 'Enter') {
            event.preventDefault();
            submit();
        }
    }

    function handleTextareaKeydown(event: KeyboardEvent) {
        const textarea = event.target as HTMLTextAreaElement;

        // Let Escape bubble up to close terminal
        if (event.key === 'Escape') {
            return;
        }

        // Let Ctrl+Enter / Cmd+Enter bubble up to submit code
        if ((event.ctrlKey || event.metaKey) && event.key === 'Enter') {
            return;
        }

        // Stop propagation for ALL other keys to prevent interference with typing
        // This is critical - without this, keys can trigger game actions or browser shortcuts
        event.stopPropagation();

        // Handle Tab - insert indent
        if (event.key === 'Tab' && !event.shiftKey) {
            event.preventDefault();
            const start = textarea.selectionStart;
            const end = textarea.selectionEnd;
            const indent = '    ';

            // If text is selected, indent all selected lines
            if (start !== end) {
                const lineStart = code.lastIndexOf('\n', start - 1) + 1;
                const selectedText = code.substring(lineStart, end);
                const indentedText = indent + selectedText.replace(/\n/g, '\n' + indent);
                code = code.substring(0, lineStart) + indentedText + code.substring(end);
                tick().then(() => {
                    textarea.selectionStart = start + indent.length;
                    textarea.selectionEnd = lineStart + indentedText.length;
                });
            } else {
                code = code.substring(0, start) + indent + code.substring(end);
                tick().then(() => {
                    textarea.selectionStart = textarea.selectionEnd = start + indent.length;
                });
            }
            return;
        }

        // Handle Shift+Tab - remove indent from current/selected lines
        if (event.key === 'Tab' && event.shiftKey) {
            event.preventDefault();
            const start = textarea.selectionStart;
            const end = textarea.selectionEnd;
            const lineStart = code.lastIndexOf('\n', start - 1) + 1;

            // Find leading whitespace (up to 4 spaces or 1 tab)
            const lineContent = code.substring(lineStart);
            const indentMatch = lineContent.match(/^( {1,4}|\t)/);

            if (indentMatch) {
                const indentLen = indentMatch[0].length;
                code = code.substring(0, lineStart) + code.substring(lineStart + indentLen);
                tick().then(() => {
                    const newStart = Math.max(lineStart, start - indentLen);
                    const newEnd = Math.max(lineStart, end - indentLen);
                    textarea.selectionStart = newStart;
                    textarea.selectionEnd = newEnd;
                });
            }
            return;
        }

        // Enter - let native behavior work (no custom handling to avoid timing issues)
        // This ensures compatibility with automated testing and paste operations
        if (event.key === 'Enter') {
            // Just let the native Enter behavior work - no preventDefault
            return;
        }

        // Handle Backspace at start of indented line - remove one indent level
        if (event.key === 'Backspace' && !event.ctrlKey && !event.metaKey) {
            const start = textarea.selectionStart;
            const end = textarea.selectionEnd;

            // Only special handling if no selection and cursor is in leading whitespace
            if (start === end && start > 0) {
                const lineStart = code.lastIndexOf('\n', start - 1) + 1;
                const beforeCursor = code.substring(lineStart, start);

                // If cursor is in leading whitespace, delete back to previous indent level
                if (beforeCursor.length > 0 && /^\s+$/.test(beforeCursor)) {
                    const deleteCount = beforeCursor.length % 4 || 4;
                    if (deleteCount > 1 && beforeCursor.length >= deleteCount) {
                        event.preventDefault();
                        code = code.substring(0, start - deleteCount) + code.substring(start);
                        tick().then(() => {
                            textarea.selectionStart = textarea.selectionEnd = start - deleteCount;
                        });
                        return;
                    }
                }
            }
            // Let default backspace behavior handle other cases
            return;
        }

        // Handle Home - go to start of text on line (after indentation), or start of line
        if (event.key === 'Home' && !event.ctrlKey && !event.metaKey) {
            event.preventDefault();
            const start = textarea.selectionStart;
            const lineStart = code.lastIndexOf('\n', start - 1) + 1;
            const lineContent = code.substring(lineStart);
            const indentMatch = lineContent.match(/^(\s*)/);
            const textStart = lineStart + (indentMatch ? indentMatch[1].length : 0);

            // Toggle between text start and line start
            const newPos = start === textStart ? lineStart : textStart;

            tick().then(() => {
                if (event.shiftKey) {
                    textarea.selectionEnd = textarea.selectionStart;
                    textarea.selectionStart = newPos;
                } else {
                    textarea.selectionStart = textarea.selectionEnd = newPos;
                }
            });
            return;
        }
    }
</script>

<svelte:window on:keydown={handleKeydown} />

<!-- Modal Overlay -->
<div class="fixed inset-0 z-50 flex items-center justify-center bg-slate-950/90">
    <!-- Grimoire/Spellbook Terminal -->
    <div class="grimoire-container {isSuccess ? 'success' : ''} {isFailure ? 'failure' : ''}">
        <!-- Header - Grimoire Title Bar -->
        <div class="grimoire-header">
            <div class="flex items-center gap-3">
                <span class="text-amber-400 text-lg">&#9733;</span>
                <h2 class="grimoire-title">SPELL CODEX</h2>
            </div>
            <div class="flex items-center gap-3">
                <span class="status-badge {busy ? 'casting' : isSuccess ? 'success' : isFailure ? 'failure' : ''}">
                    {statusLabel}
                </span>
                <button
                    onclick={handleClose}
                    class="close-btn"
                    aria-label="Close terminal"
                >
                    &#10005;
                </button>
            </div>
        </div>

        <!-- Body -->
        <div class="grimoire-body">
            <!-- Challenge Scroll -->
            {#if challenge}
                <div class="quest-scroll">
                    <div class="flex items-start justify-between gap-4">
                        <div class="flex-1">
                            <h3 class="quest-title">&#9876; QUEST</h3>
                            <p class="quest-text">{challenge}</p>
                            {#if expectedOutput}
                                <div class="expected-output">
                                    <span class="label">Expected: </span>
                                    <code class="value">{expectedOutput}</code>
                                </div>
                            {/if}
                        </div>
                        {#if onRequestHint}
                            <button
                                onclick={onRequestHint}
                                disabled={loadingHint}
                                class="hint-btn"
                            >
                                {loadingHint ? '...' : '? Hint'}
                            </button>
                        {/if}
                    </div>
                </div>
            {/if}

            <!-- Revealed Hints -->
            {#if hints.length > 0}
                <div class="hints-container">
                    {#each hints as hintText, i}
                        <div class="hint-box">
                            <span class="hint-label">Hint {i + 1}:</span> {hintText}
                        </div>
                    {/each}
                </div>
            {/if}

            <!-- Code Editor Area -->
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div
                class="code-parchment"
                onclick={(e) => {
                    // Only focus if clicking the parchment itself, not child elements
                    e.stopPropagation();
                    textareaRef?.focus();
                }}
            >
                <pre
                    class="code-highlight"
                    aria-hidden="true"
                ><code>{@html highlighted}</code></pre>

                <textarea
                    bind:this={textareaRef}
                    class="code-input"
                    spellcheck="false"
                    bind:value={code}
                    {placeholder}
                    onkeydown={handleTextareaKeydown}
                    onfocus={() => console.log('[CodeTerminal] textarea focused')}
                    onblur={(e) => {
                        const related = e.relatedTarget as HTMLElement | null;
                        console.log('[CodeTerminal] textarea blurred, focus went to:', related?.tagName, related?.className);
                        // If focus went to something inside the modal that's not a button/input, refocus textarea
                        if (related && !['BUTTON', 'INPUT', 'A'].includes(related.tagName)) {
                            setTimeout(() => textareaRef?.focus(), 0);
                        }
                    }}
                    tabindex={0}
                ></textarea>

                {#if submitting && !output}
                    <div class="casting-overlay">
                        <div class="casting-spinner">&#9733;</div>
                        <span class="casting-text">Casting spell...</span>
                    </div>
                {/if}
            </div>

            <!-- Footer Controls -->
            <div class="grimoire-footer">
                <p class="controls-hint">
                    <kbd class="key">ESC</kbd> close |
                    <kbd class="key">Ctrl+Enter</kbd> cast
                </p>
                <button
                    onclick={submit}
                    disabled={submitting && !output}
                    class="cast-btn {isSuccess ? 'success' : ''}"
                >
                    {submitting && !output ? 'CASTING...' : 'CAST SPELL'}
                </button>
            </div>

            <!-- Output Panel -->
            <div class="output-panel">
                <div class="output-header">
                    <span>&#9998; OUTPUT</span>
                    {#if isSuccess}
                        <span class="output-status success">Spell worked!</span>
                    {:else if isFailure}
                        <span class="output-status failure">Spell fizzled...</span>
                    {/if}
                </div>

                {#if output}
                    {#if output.compile_error}
                        <div class="output-box error">
                            {output.compile_error}
                        </div>
                    {/if}
                    {#if output.stdout}
                        <div class="output-box stdout">
                            {output.stdout}
                        </div>
                    {/if}
                    {#if output.stderr}
                        <div class="output-box warning">
                            {output.stderr}
                        </div>
                    {/if}
                    {#if output.message}
                        <p class="output-message">{output.message}</p>
                    {/if}
                {:else}
                    <p class="output-empty">Awaiting incantation...</p>
                {/if}
            </div>
        </div>
    </div>
</div>

<style>
    /* Grimoire Container - Pixel Art Style */
    .grimoire-container {
        background: linear-gradient(180deg, #1a1a2e 0%, #0f0f1a 100%);
        border: 4px solid #3a506b;
        border-top-color: #5a7090;
        border-left-color: #5a7090;
        box-shadow:
            inset 0 0 0 2px #0a0a1e,
            8px 8px 0 #050510;
        width: 100%;
        max-width: 900px;
        max-height: 90vh;
        overflow: hidden;
        display: flex;
        flex-direction: column;
    }

    .grimoire-container.success {
        border-color: #22c55e;
        border-top-color: #4ade80;
        border-left-color: #4ade80;
    }

    .grimoire-container.failure {
        border-color: #ef4444;
        border-top-color: #f87171;
        border-left-color: #f87171;
        animation: shake 0.3s ease-out;
    }

    @keyframes shake {
        0%, 100% { transform: translateX(0); }
        25% { transform: translateX(-4px); }
        75% { transform: translateX(4px); }
    }

    /* Header */
    .grimoire-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 12px 16px;
        background: linear-gradient(180deg, #16213e 0%, #0f3460 100%);
        border-bottom: 3px solid #0a0a1e;
    }

    .grimoire-title {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 12px;
        color: #fbbf24;
        text-shadow: 2px 2px 0 #92400e;
        letter-spacing: 2px;
    }

    .status-badge {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 8px;
        padding: 4px 8px;
        background: #1a1a2e;
        border: 2px solid #3a506b;
        color: #94a3b8;
    }

    .status-badge.casting {
        color: #fbbf24;
        border-color: #fbbf24;
        animation: pulse 1s infinite;
    }

    .status-badge.success {
        color: #4ade80;
        border-color: #22c55e;
    }

    .status-badge.failure {
        color: #f87171;
        border-color: #ef4444;
    }

    @keyframes pulse {
        0%, 100% { opacity: 1; }
        50% { opacity: 0.5; }
    }

    .close-btn {
        font-size: 16px;
        color: #64748b;
        background: none;
        border: none;
        cursor: pointer;
        padding: 4px 8px;
    }

    .close-btn:hover {
        color: #f87171;
    }

    /* Body */
    .grimoire-body {
        padding: 16px;
        overflow-y: auto;
        flex: 1;
    }

    /* Quest Scroll */
    .quest-scroll {
        background: linear-gradient(180deg, #1e3a5f 0%, #0f3460 100%);
        border: 3px solid #3a506b;
        border-top-color: #5a7090;
        border-left-color: #5a7090;
        padding: 12px 16px;
        margin-bottom: 12px;
    }

    .quest-title {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 10px;
        color: #67e8f9;
        margin-bottom: 8px;
    }

    .quest-text {
        font-size: 13px;
        color: #e2e8f0;
        line-height: 1.5;
    }

    .expected-output {
        margin-top: 8px;
        padding: 6px 10px;
        background: #0a0a1e;
        border: 2px solid #1e293b;
    }

    .expected-output .label {
        font-size: 11px;
        color: #64748b;
    }

    .expected-output .value {
        font-family: 'IBM Plex Mono', monospace;
        font-size: 12px;
        color: #4ade80;
    }

    .hint-btn {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 8px;
        padding: 6px 10px;
        background: linear-gradient(180deg, #92400e 0%, #78350f 100%);
        border: 2px solid #fbbf24;
        border-bottom-color: #92400e;
        border-right-color: #92400e;
        color: #fef3c7;
        cursor: pointer;
        white-space: nowrap;
    }

    .hint-btn:hover {
        background: linear-gradient(180deg, #a3540f 0%, #854d0e 100%);
    }

    .hint-btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    /* Hints */
    .hints-container {
        margin-bottom: 12px;
        display: flex;
        flex-direction: column;
        gap: 6px;
    }

    .hint-box {
        background: linear-gradient(180deg, #78350f 0%, #92400e 100%);
        border: 2px solid #fbbf24;
        padding: 8px 12px;
        font-size: 11px;
        color: #fef3c7;
    }

    .hint-label {
        font-weight: bold;
        color: #fcd34d;
    }

    /* Code Editor Parchment */
    .code-parchment {
        position: relative;
        background: #0a0a14;
        border: 3px solid #1e293b;
        border-top-color: #0a0a0e;
        border-left-color: #0a0a0e;
        padding: 12px;
        min-height: 280px;
        overflow: hidden;
    }

    .code-highlight {
        position: absolute;
        inset: 12px;
        margin: 0;
        overflow: auto;
        white-space: pre-wrap;
        font-family: 'IBM Plex Mono', 'Courier New', monospace;
        font-size: 13px;
        line-height: 1.6;
        color: #e2e8f0;
        pointer-events: none;
        z-index: 1;
    }

    .code-input {
        position: absolute;
        inset: 12px;
        width: calc(100% - 24px);
        height: calc(100% - 24px);
        resize: none;
        /* Use rgba with tiny alpha to ensure clickability while appearing transparent */
        background: rgba(0, 0, 0, 0.001);
        font-family: 'IBM Plex Mono', 'Courier New', monospace;
        font-size: 13px;
        line-height: 1.6;
        color: transparent;
        caret-color: #fbbf24;
        outline: none;
        border: none;
        z-index: 2;
        cursor: text;
        /* Ensure it receives all pointer events */
        pointer-events: auto;
    }

    .code-input::placeholder {
        color: #475569;
    }

    .casting-overlay {
        position: absolute;
        inset: 0;
        background: rgba(10, 10, 20, 0.85);
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 12px;
    }

    .casting-spinner {
        font-size: 32px;
        color: #fbbf24;
        animation: spin 1s linear infinite;
    }

    @keyframes spin {
        from { transform: rotate(0deg); }
        to { transform: rotate(360deg); }
    }

    .casting-text {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 10px;
        color: #fbbf24;
    }

    /* Footer */
    .grimoire-footer {
        display: flex;
        align-items: center;
        justify-content: space-between;
        margin-top: 12px;
        padding-top: 12px;
        border-top: 2px solid #1e293b;
    }

    .controls-hint {
        font-size: 11px;
        color: #64748b;
    }

    .key {
        display: inline-block;
        background: #1a1a2e;
        border: 2px solid #3a506b;
        border-bottom-color: #1e293b;
        border-right-color: #1e293b;
        padding: 2px 6px;
        font-family: 'IBM Plex Mono', monospace;
        font-size: 10px;
        color: #94a3b8;
        margin: 0 2px;
    }

    .cast-btn {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 10px;
        padding: 10px 20px;
        background: linear-gradient(180deg, #1e40af 0%, #1e3a8a 100%);
        border: 3px solid #3b82f6;
        border-bottom-color: #1e40af;
        border-right-color: #1e40af;
        box-shadow: 4px 4px 0 #0a0a1e;
        color: #dbeafe;
        cursor: pointer;
        transition: transform 0.1s;
    }

    .cast-btn:hover {
        transform: translate(2px, 2px);
        box-shadow: 2px 2px 0 #0a0a1e;
    }

    .cast-btn:active {
        transform: translate(4px, 4px);
        box-shadow: none;
    }

    .cast-btn:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .cast-btn.success {
        background: linear-gradient(180deg, #166534 0%, #14532d 100%);
        border-color: #22c55e;
        border-bottom-color: #166534;
        border-right-color: #166534;
    }

    /* Output Panel */
    .output-panel {
        margin-top: 12px;
        background: #0a0a14;
        border: 3px solid #1e293b;
        border-top-color: #0a0a0e;
        border-left-color: #0a0a0e;
        padding: 12px;
    }

    .output-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 9px;
        color: #64748b;
        margin-bottom: 8px;
    }

    .output-status.success {
        color: #4ade80;
    }

    .output-status.failure {
        color: #f87171;
    }

    .output-box {
        font-family: 'IBM Plex Mono', monospace;
        font-size: 12px;
        padding: 8px 10px;
        margin-top: 6px;
        white-space: pre-wrap;
        word-break: break-word;
    }

    .output-box.stdout {
        background: #0f172a;
        border: 2px solid #1e293b;
        color: #e2e8f0;
    }

    .output-box.error {
        background: #450a0a;
        border: 2px solid #7f1d1d;
        color: #fecaca;
    }

    .output-box.warning {
        background: #451a03;
        border: 2px solid #78350f;
        color: #fed7aa;
    }

    .output-message {
        font-size: 11px;
        color: #64748b;
        margin-top: 8px;
    }

    .output-empty {
        font-size: 11px;
        color: #475569;
        font-style: italic;
    }

    /* Syntax highlighting */
    :global(.hl-keyword) {
        color: #67e8f9;
    }
    :global(.hl-number) {
        color: #fcd34d;
    }
    :global(.hl-string) {
        color: #6ee7b7;
    }
    :global(.hl-comment) {
        color: #64748b;
    }
</style>
