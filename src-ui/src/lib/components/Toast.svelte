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

    function getBorderColor(type: string): string {
        switch (type) {
            case 'success':
                return 'border-emerald-500/50';
            case 'error':
                return 'border-rose-500/50';
            default:
                return 'border-cyan-500/50';
        }
    }

    function getGlowColor(type: string): string {
        switch (type) {
            case 'success':
                return 'shadow-emerald-500/20';
            case 'error':
                return 'shadow-rose-500/20';
            default:
                return 'shadow-cyan-500/20';
        }
    }

    function getTextColor(type: string): string {
        switch (type) {
            case 'success':
                return 'text-emerald-300';
            case 'error':
                return 'text-rose-300';
            default:
                return 'text-cyan-300';
        }
    }
</script>

<div class="pointer-events-none fixed bottom-4 right-4 z-50 flex flex-col gap-3">
    {#each messages as toast (toast.id)}
        <div
            class="pointer-events-auto animate-slide-in rounded-lg border-2 bg-slate-900/95 p-4 shadow-lg backdrop-blur-sm transition-all {getBorderColor(
                toast.type
            )} {getGlowColor(toast.type)}"
        >
            <div class="flex items-start justify-between gap-3">
                <div class="flex-1">
                    <p class="font-semibold {getTextColor(toast.type)}">{toast.message}</p>
                    {#if toast.details}
                        <p class="mt-1 text-xs font-mono text-slate-400 whitespace-pre-wrap">
                            {toast.details}
                        </p>
                    {/if}
                </div>
                <button
                    onclick={() => dismiss(toast.id)}
                    class="text-slate-400 hover:text-slate-200 transition-colors"
                    aria-label="Dismiss"
                >
                    <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M6 18L18 6M6 6l12 12"
                        />
                    </svg>
                </button>
            </div>

            <!-- Scan line effect -->
            <div class="absolute inset-0 pointer-events-none overflow-hidden rounded-lg">
                <div class="scan-line"></div>
            </div>
        </div>
    {/each}
</div>

<style>
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

    @keyframes scan {
        from {
            transform: translateY(-100%);
        }
        to {
            transform: translateY(200%);
        }
    }

    .animate-slide-in {
        animation: slide-in 0.3s ease-out;
    }

    .scan-line {
        position: absolute;
        width: 100%;
        height: 50%;
        background: linear-gradient(
            to bottom,
            transparent,
            rgba(34, 211, 238, 0.03),
            transparent
        );
        animation: scan 3s linear infinite;
    }
</style>
