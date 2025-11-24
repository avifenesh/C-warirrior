import { a as attr, s as slot, b as attr_class, c as attr_style, d as stringify, e as ensure_array_like, f as store_get, h as head, u as unsubscribe_stores } from "../../chunks/index2.js";
import { Y as ssr_context, X as escape_html } from "../../chunks/context.js";
import "clsx";
import { d as derived, w as writable } from "../../chunks/index.js";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
function html(value) {
  var html2 = String(value ?? "");
  var open = "<!---->";
  return open + html2 + "<!---->";
}
function onDestroy(fn) {
  /** @type {SSRContext} */
  ssr_context.r.on_destroy(fn);
}
const DEFAULT_STATUS = "Waiting for backend tick...";
function createGameStore() {
  const renderState = writable(null);
  const ui = writable({
    loading: true,
    status: "Booting Code Warrior...",
    error: null
  });
  const levels = writable([]);
  const lastCodeOutput = writable(null);
  const lastLevelComplete = writable(null);
  const codeSubmitting = writable(false);
  const lastCodeResult = writable(null);
  let tickUnsub = null;
  let errorUnsub = null;
  let codeUnsub = null;
  let levelCompleteUnsub = null;
  async function boot() {
    ui.update((u) => ({
      ...u,
      loading: true,
      status: "Resetting backend state...",
      error: null
    }));
    try {
      await invoke("init_game");
      ui.update((u) => ({ ...u, status: DEFAULT_STATUS }));
      await hydrateLevels();
      await bindEvents();
    } catch (err) {
      ui.update((u) => ({ ...u, error: normalizeError(err) }));
    } finally {
      ui.update((u) => ({ ...u, loading: false }));
    }
  }
  async function hydrateLevels() {
    try {
      const data = await invoke("get_available_levels");
      levels.set(data);
    } catch (err) {
      ui.update((u) => ({ ...u, error: u.error ?? normalizeError(err) }));
    }
  }
  async function bindEvents() {
    if (!tickUnsub) {
      tickUnsub = await listen("game_tick", (event) => {
        renderState.set(event.payload);
        ui.update((u) => ({ ...u, status: "Live" }));
      });
    }
    if (!errorUnsub) {
      errorUnsub = await listen("game_error", (event) => {
        ui.update((u) => ({ ...u, error: event.payload.message }));
      });
    }
    if (!codeUnsub) {
      codeUnsub = await listen("code_output", (event) => {
        lastCodeOutput.set(event.payload);
      });
    }
    if (!levelCompleteUnsub) {
      levelCompleteUnsub = await listen("level_complete", (event) => {
        lastLevelComplete.set(event.payload);
      });
    }
  }
  async function startLevel(levelId) {
    ui.update((u) => ({
      ...u,
      loading: true,
      status: `Loading ${levelId}...`,
      error: null
    }));
    try {
      await invoke("load_level", { levelId });
      ui.update((u) => ({ ...u, status: `Level ${levelId} loaded` }));
    } catch (err) {
      ui.update((u) => ({ ...u, error: normalizeError(err) }));
    } finally {
      ui.update((u) => ({ ...u, loading: false }));
    }
  }
  async function sendAction(action) {
    ui.update((u) => ({ ...u, error: null }));
    try {
      const next = await invoke("process_action", { action });
      renderState.set(next);
    } catch (err) {
      ui.update((u) => ({ ...u, error: normalizeError(err) }));
    }
  }
  async function submitCode(code) {
    ui.update((u) => ({ ...u, error: null }));
    codeSubmitting.set(true);
    try {
      const result = await invoke("submit_code", { code });
      lastCodeResult.set(result);
    } catch (err) {
      ui.update((u) => ({ ...u, error: normalizeError(err) }));
    } finally {
      codeSubmitting.set(false);
    }
  }
  const phase = derived(renderState, ($rs) => $rs?.game_phase ?? "main_menu");
  const currentLevelId = derived(renderState, ($rs) => $rs?.current_level_id ?? null);
  function cleanup() {
    tickUnsub?.();
    errorUnsub?.();
    codeUnsub?.();
    levelCompleteUnsub?.();
  }
  return {
    renderState,
    ui,
    levels,
    lastCodeOutput,
    lastLevelComplete,
    phase,
    currentLevelId,
    codeSubmitting,
    lastCodeResult,
    boot,
    startLevel,
    sendAction,
    submitCode,
    refreshLevels: hydrateLevels,
    cleanup
  };
}
function normalizeError(err) {
  if (err instanceof Error) return err.message;
  return typeof err === "string" ? err : "Unknown error";
}
const gameStore = createGameStore();
function GameWorld($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let { renderState = null, tileSize = 32 } = $$props;
    function handleKeydown(event) {
      const key = event.key.toLowerCase();
      const movementMapping = {
        w: "up",
        arrowup: "up",
        a: "left",
        arrowleft: "left",
        s: "down",
        arrowdown: "down",
        d: "right",
        arrowright: "right"
      };
      if (movementMapping[key]) {
        event.preventDefault();
      }
      if (key === "e" || key === " ") {
        event.preventDefault();
      }
    }
    onDestroy(() => {
      window.removeEventListener("keydown", handleKeydown);
    });
    $$renderer2.push(`<div class="relative min-h-screen w-full bg-slate-950 outline-none svelte-ec6f7e" tabindex="-1" role="application" aria-label="Game world"><div class="flex items-center justify-center min-h-screen p-8 svelte-ec6f7e"><div class="relative svelte-ec6f7e"><canvas${attr("width", tileSize * 20)}${attr("height", tileSize * 15)} class="rounded-2xl border-2 border-slate-800 shadow-2xl shadow-slate-950/50" aria-label="Game viewport"></canvas> <div class="pointer-events-none absolute inset-0 rounded-2xl svelte-ec6f7e" style="box-shadow: inset 0 0 100px 40px rgba(2, 6, 23, 0.5);"></div></div></div> <!--[-->`);
    slot($$renderer2, $$props, "default", {});
    $$renderer2.push(`<!--]--></div>`);
  });
}
function CodeTerminal($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let {
      initialCode = "",
      placeholder = "// Cast your spell...",
      submitting = false
    } = $$props;
    let code = initialCode;
    const keywordPattern = /\b(int|float|double|char|void|return|if|else|for|while|do|switch|case|break|continue|struct|typedef|const|printf|scanf|malloc|free|sizeof|include|define)\b/g;
    const numberPattern = /\b(0x[\da-fA-F]+|\d+(?:\.\d+)?)\b/g;
    const stringPattern = /"(?:[^"\\]|\\.)*"/g;
    const commentPattern = /\/\/.*$/gm;
    function escapeHtml(str) {
      return str.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
    }
    function highlight(text) {
      let out = escapeHtml(text);
      out = out.replace(commentPattern, '<span class="text-slate-500">$&</span>');
      out = out.replace(stringPattern, '<span class="text-emerald-300">$&</span>');
      out = out.replace(keywordPattern, '<span class="text-cyan-300">$1</span>');
      out = out.replace(numberPattern, '<span class="text-amber-300">$1</span>');
      return out;
    }
    const highlighted = highlight(code);
    $$renderer2.push(`<div class="fixed inset-0 z-50 flex items-center justify-center bg-slate-950/80 backdrop-blur-sm svelte-1hxo5d7"><div class="relative w-full max-w-4xl rounded-2xl border-2 border-cyan-500/50 bg-slate-900/95 shadow-2xl shadow-cyan-500/20 backdrop-blur-md svelte-1hxo5d7"><div class="flex items-center justify-between border-b border-cyan-500/30 bg-slate-950/50 px-6 py-4 svelte-1hxo5d7"><div class="flex items-center gap-3 svelte-1hxo5d7"><div class="flex gap-2 svelte-1hxo5d7"><div class="h-3 w-3 rounded-full bg-rose-500/50 svelte-1hxo5d7"></div> <div class="h-3 w-3 rounded-full bg-amber-500/50 svelte-1hxo5d7"></div> <div class="h-3 w-3 rounded-full bg-emerald-500/50 svelte-1hxo5d7"></div></div> <h2 class="font-mono text-sm uppercase tracking-widest text-cyan-400 svelte-1hxo5d7">// Code Terminal</h2></div> <button class="text-slate-400 hover:text-slate-200 transition-colors svelte-1hxo5d7" aria-label="Close terminal"><svg class="h-6 w-6 svelte-1hxo5d7" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" class="svelte-1hxo5d7"></path></svg></button></div> <div class="p-6 svelte-1hxo5d7"><div class="relative rounded-xl border-2 border-slate-800 bg-slate-950 p-4 shadow-inner svelte-1hxo5d7"><pre class="pointer-events-none absolute inset-4 m-0 overflow-auto whitespace-pre-wrap font-mono text-sm leading-relaxed text-slate-200 svelte-1hxo5d7" aria-hidden="true"><code class="svelte-1hxo5d7">${html(highlighted)}</code></pre> <textarea class="relative h-96 w-full resize-none bg-transparent font-mono text-sm text-transparent caret-cyan-300 outline-none svelte-1hxo5d7" spellcheck="false"${attr("placeholder", placeholder)} autofocus>`);
    const $$body = escape_html(code);
    if ($$body) {
      $$renderer2.push(`${$$body}`);
    }
    $$renderer2.push(`</textarea> <div class="absolute inset-0 pointer-events-none overflow-hidden rounded-xl svelte-1hxo5d7"><div class="terminal-scan-line svelte-1hxo5d7"></div></div></div> <div class="mt-4 flex items-center justify-between svelte-1hxo5d7"><p class="text-xs font-mono text-slate-500 svelte-1hxo5d7"><kbd class="px-2 py-1 rounded bg-slate-800 text-cyan-400 svelte-1hxo5d7">ESC</kbd> to close | <kbd class="px-2 py-1 rounded bg-slate-800 text-cyan-400 svelte-1hxo5d7">Ctrl+Enter</kbd> to cast spell</p> <button${attr("disabled", submitting, true)} class="rounded-lg border-2 border-cyan-500/50 bg-cyan-500/10 px-6 py-2 font-semibold text-cyan-300 transition-all hover:border-cyan-400 hover:bg-cyan-500/20 hover:text-cyan-200 disabled:opacity-50 disabled:cursor-not-allowed shadow-lg shadow-cyan-500/20 svelte-1hxo5d7">${escape_html(submitting ? "Casting Spell..." : "⚡ Cast Spell")}</button></div></div> <div class="absolute inset-0 pointer-events-none rounded-2xl svelte-1hxo5d7"><div class="glow-pulse svelte-1hxo5d7"></div></div></div></div>`);
  });
}
function GameHUD($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let { player, currentLevelId } = $$props;
    let healthPercent = player ? Math.max(0, Math.min(100, player.health / player.max_health * 100)) : 0;
    let xpPercent = player ? player.xp % 100 : 0;
    function getHealthColor(percent) {
      if (percent > 66) return "bg-emerald-500";
      if (percent > 33) return "bg-amber-500";
      return "bg-rose-500";
    }
    function getHealthGlow(percent) {
      if (percent > 66) return "shadow-emerald-500/50";
      if (percent > 33) return "shadow-amber-500/50";
      return "shadow-rose-500/50";
    }
    $$renderer2.push(`<div class="pointer-events-none fixed left-4 top-4 z-40 space-y-3"><div class="rounded-lg border border-cyan-500/30 bg-slate-900/80 px-4 py-2 backdrop-blur-sm shadow-lg shadow-cyan-500/10"><p class="text-xs uppercase tracking-widest text-cyan-400">Level</p> <p class="text-2xl font-bold text-cyan-200">${escape_html(currentLevelId ?? "--")}</p></div> `);
    if (player) {
      $$renderer2.push("<!--[-->");
      $$renderer2.push(`<div class="rounded-lg border border-slate-700/50 bg-slate-900/80 p-3 backdrop-blur-sm shadow-lg"><div class="mb-2 flex items-center justify-between"><p class="text-xs uppercase tracking-widest text-slate-400">Health</p> <p class="text-xs font-mono text-slate-300">${escape_html(player.health)}/${escape_html(player.max_health)}</p></div> <div class="h-3 w-48 overflow-hidden rounded-full bg-slate-800 border border-slate-700"><div${attr_class(`h-full transition-all duration-300 shadow-lg ${stringify(getHealthColor(healthPercent))} ${stringify(getHealthGlow(healthPercent))}`, "svelte-7ggbmv")}${attr_style(`width: ${stringify(healthPercent)}%`)}></div></div></div> <div class="rounded-lg border border-slate-700/50 bg-slate-900/80 p-3 backdrop-blur-sm shadow-lg"><div class="mb-2 flex items-center justify-between"><p class="text-xs uppercase tracking-widest text-slate-400">XP <span class="ml-1 text-cyan-400">Lv.${escape_html(player.level)}</span></p> <p class="text-xs font-mono text-slate-300">${escape_html(xpPercent)}/100</p></div> <div class="h-3 w-48 overflow-hidden rounded-full bg-slate-800 border border-slate-700"><div class="h-full bg-gradient-to-r from-cyan-500 to-blue-500 transition-all duration-300 shadow-lg shadow-cyan-500/50"${attr_style(`width: ${stringify(xpPercent)}%`)}></div></div></div>`);
    } else {
      $$renderer2.push("<!--[!-->");
    }
    $$renderer2.push(`<!--]--> <div class="absolute inset-0 pointer-events-none overflow-hidden rounded-lg"><div class="hud-scan-line svelte-7ggbmv"></div></div></div>`);
  });
}
function Toast($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let { messages = [] } = $$props;
    function getBorderColor(type) {
      switch (type) {
        case "success":
          return "border-emerald-500/50";
        case "error":
          return "border-rose-500/50";
        default:
          return "border-cyan-500/50";
      }
    }
    function getGlowColor(type) {
      switch (type) {
        case "success":
          return "shadow-emerald-500/20";
        case "error":
          return "shadow-rose-500/20";
        default:
          return "shadow-cyan-500/20";
      }
    }
    function getTextColor(type) {
      switch (type) {
        case "success":
          return "text-emerald-300";
        case "error":
          return "text-rose-300";
        default:
          return "text-cyan-300";
      }
    }
    $$renderer2.push(`<div class="pointer-events-none fixed bottom-4 right-4 z-50 flex flex-col gap-3"><!--[-->`);
    const each_array = ensure_array_like(messages);
    for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
      let toast = each_array[$$index];
      $$renderer2.push(`<div${attr_class(`pointer-events-auto animate-slide-in rounded-lg border-2 bg-slate-900/95 p-4 shadow-lg backdrop-blur-sm transition-all ${stringify(getBorderColor(toast.type))} ${stringify(getGlowColor(toast.type))}`, "svelte-1cpok13")}><div class="flex items-start justify-between gap-3"><div class="flex-1"><p${attr_class(`font-semibold ${stringify(getTextColor(toast.type))}`, "svelte-1cpok13")}>${escape_html(toast.message)}</p> `);
      if (toast.details) {
        $$renderer2.push("<!--[-->");
        $$renderer2.push(`<p class="mt-1 text-xs font-mono text-slate-400 whitespace-pre-wrap">${escape_html(toast.details)}</p>`);
      } else {
        $$renderer2.push("<!--[!-->");
      }
      $$renderer2.push(`<!--]--></div> <button class="text-slate-400 hover:text-slate-200 transition-colors" aria-label="Dismiss"><svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg></button></div> <div class="absolute inset-0 pointer-events-none overflow-hidden rounded-lg"><div class="scan-line svelte-1cpok13"></div></div></div>`);
    }
    $$renderer2.push(`<!--]--></div>`);
  });
}
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    var $$store_subs;
    const game = gameStore;
    let codeDraft = '// Write your C spell here...\n#include <stdio.h>\n\nint main() {\n    printf("Hello, World!\\n");\n    return 0;\n}';
    let toastMessages = [];
    const { renderState, currentLevelId, codeSubmitting } = game;
    let showTerminal = store_get($$store_subs ??= {}, "$renderState", renderState)?.show_terminal ?? false;
    onDestroy(() => {
      game.cleanup();
    });
    head("1uha8ag", $$renderer2, ($$renderer3) => {
      $$renderer3.title(($$renderer4) => {
        $$renderer4.push(`<title>Code Warrior: C Mastery</title>`);
      });
    });
    GameWorld($$renderer2, {
      renderState: store_get($$store_subs ??= {}, "$renderState", renderState),
      children: ($$renderer3) => {
        GameHUD($$renderer3, {
          player: store_get($$store_subs ??= {}, "$renderState", renderState)?.player ?? null,
          currentLevelId: store_get($$store_subs ??= {}, "$currentLevelId", currentLevelId)
        });
        $$renderer3.push(`<!----> `);
        if (showTerminal) {
          $$renderer3.push("<!--[-->");
          CodeTerminal($$renderer3, {
            initialCode: codeDraft,
            submitting: store_get($$store_subs ??= {}, "$codeSubmitting", codeSubmitting)
          });
        } else {
          $$renderer3.push("<!--[!-->");
        }
        $$renderer3.push(`<!--]--> `);
        Toast($$renderer3, { messages: toastMessages });
        $$renderer3.push(`<!---->`);
      },
      $$slots: { default: true }
    });
    if ($$store_subs) unsubscribe_stores($$store_subs);
  });
}
export {
  _page as default
};
