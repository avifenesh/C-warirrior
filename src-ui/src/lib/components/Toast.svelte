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
            class="pixel-toast pointer-events-auto animate-slide-in {toast.type} shadow-md"
        >
            <div class="flex items-start justify-between gap-2">
                <div class="flex-1">
                    <p class="flex items-center">
                        <span class="pixel-icon">{getIcon(toast.type)}</span>
                        <span class="pixel-text">{toast.message}</span>
                    </p>
                    {#if toast.details}
                        <p class="mt-1 text-[8px] font-mono text-slate-300 whitespace-pre-wrap opacity-80 pl-4">
                            {toast.details}
                        </p>
                    {/if}
                </div>
                <button
                    onclick={() => dismiss(toast.id)}
                    class="pixel-close-btn"
                    aria-label="Dismiss"
                >
                    ✕
                </button>
            </div>
        </div>
    {/each}
</div>

<style lang="postcss">
    /* Pixel panel style matching other UI elements */
    .pixel-toast {
        font-family: 'Press Start 2P', cursive;
        font-size: 8px;
        padding: 8px 12px;
        margin-top: 8px;
        background: linear-gradient(180deg, #1a1a2e 0%, #16213e 100%);
        border: 3px solid #0f3460;
        border-top-color: #3a506b;
        border-left-color: #3a506b;
        box-shadow:
            inset 0 0 0 1px #0a0a1e,
            4px 4px 0 #0a0a1e;
        min-width: 200px;
    }

    /* Success variant */
    .pixel-toast.success {
        border-color: #22c55e;
        border-top-color: #4ade80;
        border-left-color: #4ade80;
        background: linear-gradient(180deg, #14532d 0%, #166534 100%);
    }

    /* Error variant */
    .pixel-toast.error {
        border-color: #ef4444;
        border-top-color: #f87171;
        border-left-color: #f87171;
        background: linear-gradient(180deg, #7f1d1d 0%, #991b1b 100%);
    }

    /* Info variant (default) */
    .pixel-toast.info {
        /* Uses default styles */
    }

    .pixel-icon {
        margin-right: 6px;
        font-size: 10px;
    }

    .pixel-text {
        line-height: 1.4;
        color: #e2e8f0;
        text-shadow: 1px 1px 0 rgba(0,0,0,0.5);
    }
    
    .pixel-toast.success .pixel-text {
        color: #dcfce7;
    }
    
    .pixel-toast.error .pixel-text {
        color: #fee2e2;
    }

    .pixel-close-btn {
        font-family: 'Press Start 2P', cursive;
        font-size: 8px;
        background: none;
        border: none;
        cursor: pointer;
        padding: 2px 4px;
        margin-left: 8px;
        color: rgba(255,255,255,0.5);
    }
    
    .pixel-close-btn:hover {
        color: rgba(255,255,255,0.9);
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