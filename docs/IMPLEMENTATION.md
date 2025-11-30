# Code Warrior Implementation Guide

## For AI agents and developers
- Use this as the entrypoint for concrete setup steps, patterns, and tools.
- Read `docs/dev_guide/setup.md` for project setup, code examples, and development patterns.
- Read `tools/README.md` for MCP servers and procedural generation tools.
- Frontend uses Svelte 5 Runes only; no `svelte/store` remains. State is kept in-route and talks directly to the Backend abstraction (`src-ui/src/lib/backend`).
- Web build targets the Axum API endpoints listed in `docs/architecture/system.md` (HTTP contract table). The desktop build keeps using Tauri commands/events.
- Keep the HTTP/Tauri/WASM flows synchronized with `docs/logic-mindmap.md`, which maps each route/handler to its Rust and Svelte sources.

---

## Documents

- `docs/dev_guide/setup.md` – implementation guide, code examples, patterns, roadmap, troubleshooting.
- `tools/README.md` – tooling guide for the C compiler MCP server and map generator.
