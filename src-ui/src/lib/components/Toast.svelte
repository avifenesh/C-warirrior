<script lang="ts">
    import { onMount } from 'svelte';

    export interface ToastMessage {
        id: string;
        type: 'success' | 'error' | 'info';
        message: string;
        details?: string;
    }

    interface Props {
        messages: ToastMessage[];
        onDismiss?: (id: string) => void;
    }

    let { messages = [], onDismiss }: Props = $props();

    onMount(() => {
        // Auto-dismiss toasts after 3 seconds
        messages.forEach((msg) => {
            setTimeout(() => {
                if (onDismiss) {
                    onDismiss(msg.id);
                }
            }, 3000);
        });
    });

    function dismiss(id: string) {
        if (onDismiss) {
            onDismiss(id);
        }
    }

    function getPixelBorder(type: string): string {
        switch (type) {
            case 'success':
                return 'border-emerald-900 border-2 border-solid';
            case 'error':
                return 'border-rose-900 border-2 border-solid';
            default:
                return 'border-cyan-900 border-2 border-solid';
        }
    }

    function getGradientBg(type: string): string {
        switch (type) {
            case 'success':
                return 'bg-gradient-to-br from-emerald-800 to-emerald-900';
            case 'error':
                return 'bg-gradient-to-br from-rose-800 to-rose-900';
            default:
                return 'bg-gradient-to-br from-cyan-800 to-cyan-900';
        }
    }

    function getTextColor(type: string): string {
        switch (type) {
            case 'success':
                return 'text-emerald-200';
            case 'error':
                return 'text-rose-200';
            default:
                return 'text-cyan-200';
        }
    }

    function getIcon(type: string): string {
        switch (type) {
            case 'success':
                return '✔';
            case 'error':
                return '✘';
            default:
                return 'ℹ';
        }
    }
</script>

<div class="pointer-events-none fixed bottom-4 right-4 z-50 flex flex-col gap-2">
    {#each messages as toast (toast.id)}
        <div
            class="pixel-toast pointer-events-auto animate-slide-in {getPixelBorder(toast.type)} {getGradientBg(toast.type)} p-2 shadow-md"
        >
            <div class="flex items-start justify-between gap-2">
                <div class="flex-1">
                    <p class="flex items-center gap-2 {getTextColor(toast.type)}">
                        <span class="pixel-icon">{getIcon(toast.type)}</span>
                        <span class="pixel-text">{toast.message}</span>
                    </p>
                    {#if toast.details}
                        <p class="mt-1 text-[8px] font-mono text-slate-300 whitespace-pre-wrap">
                            {toast.details}
                        </p>
                    {/if}
                </div>
                <button
                    onclick={() => dismiss(toast.id)}
                    class="pixel-close-btn text-slate-400 hover:text-slate-200"
                    aria-label="Dismiss"
                >
                    ✕
                </button>
            </div>
        </div>
    {/each}
</div>

<style lang="postcss">
    /* Import Press Start 2P font */
    @import url('https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap');

    .pixel-toast {
        font-family: 'Press Start 2P', cursive;
        font-size: 8px;
        border-image-slice: 2;
        border-image-width: 2;
        box-shadow:
            -2px -2px 0 #000,
             2px -2px 0 #000,
            -2px  2px 0 #000,
             2px  2px 0 #000;
    }

    .pixel-icon {
        margin-right: 4px;
    }

    .pixel-text {
        line-height: 1.2;
    }

    .pixel-close-btn {
        font-family: 'Press Start 2P', cursive;
        font-size: 8px;
        background: none;
        border: none;
        cursor: pointer;
        padding: 0;
    }

    @keyframes slide-in {
        from {
            transform: translateX(100%);
            opacity: 0;
        }
        to {
            transform: translateX(0);
            opacity: 1;
        }
    }

    .animate-slide-in {
        animation: slide-in 0.3s ease-out;
    }
</style>