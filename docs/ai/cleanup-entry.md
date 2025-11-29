# Code Warrior Cleanup – Entry Guide for AI Agents

Goal: clean up technical debt, align desktop and web behavior, and add full Save/Load support on the HTTP backend while keeping Rust as the single source of game logic.

This work is split between two agents:
- **Backend Agent (A)** – Owns Rust core (`src/`), HTTP API (`src-api/`), Tauri layer (`src-tauri/`), and DB schema/operations.
- **Frontend Agent (B)** – Owns Svelte/TypeScript (`src-ui/`), TS backend abstraction, and wiring UI to existing backend behaviors.

## Global Objectives

- Remove dead/legacy code paths and unused commands.
- Make HTTP API and Tauri expose equivalent behavior for core gameplay (movement, levels, code submission, hints, progress).
- Align Rust and TypeScript types for `CodeResult`, events, and progress.
- Implement Save/Load on the HTTP backend and wire the existing Save/Load UI for both platforms.
- Replace noisy debug logging with structured or minimal error logging only.

## Coordination Rules (Both Agents)

- **Ownership**
  - Backend agent edits: `src/**`, `src-api/**`, `src-tauri/**`, `src-api/src/db/**`.
  - Frontend agent edits: `src-ui/**` and documentation under `docs/ai/**` (TS-facing parts).
  - **Shared TS type files** (`src-ui/src/lib/types.ts`, `src-ui/src/lib/backend/types.ts`) are owned by the frontend agent. Backend must adapt Rust types to match, not edit TS files.
- **Shared behavior specs**
  - High-level shapes for shared types (e.g. `CodeResult`, `LevelCompleteEvent`, Save slot metadata) are described in words; frontend encodes them in TS, backend mirrors them in Rust/HTTP.
- **Avoiding conflicts**
  - If backend needs a TS type change, it adds a TODO note in the frontend agent’s spec file instead of editing TS directly.
  - If frontend needs a new backend route or field, it adds a TODO note in the backend agent’s spec file instead of changing Rust.
- **Testing**
  - Backend: run appropriate `cargo test` and HTTP smoke tests after each phase that changes behavior.
  - Frontend: verify main flow on both web and Tauri after each phase, following AGENTS.md testing protocol.

## Phase Overview

Use these as high-level milestones; details live in the agent-specific files.

1. **Baseline & Mapping** – Confirm current flows on web and desktop; note existing gaps and errors without changing code.
2. **Dead Code & Interface Pruning** – Remove unused Tauri API (`lib/api.ts`, `complete_level`), and deprecated interfaces so only live paths remain.
3. **Type Alignment** – Align TS and Rust/HTTP for `CodeResult` and events (e.g. `LevelCompleteEvent`) so both backends present the same contract to the UI.
4. **Backend Parity (Core Flow & Hints)** – Add missing HTTP routes (`getLevelData`, `getHint`), fix `Resume` behavior, and ensure both backends delegate to shared game logic.
5. **Save/Load Parity** – Implement Save/Load HTTP routes and persistence, expose them via TS `Backend`, and wire `SaveLoad.svelte` for both platforms.
6. **Progress Integration** – Expose progression data from backend and feed real values into `ProgressTracker` and HUD instead of placeholders.
7. **Events & Tick (Optional)** – Clarify/implement real-time updates for Tauri and keep HTTP on polling/response model.
8. **Logging Cleanup** – Strip noisy `console.log`/`println!` debug output, keeping only meaningful error/logging via existing infra.

## Agent-Specific Specs

Each agent should read AGENTS.md, this entry file, then only its own focused spec:

- **Backend Agent (A)**: `docs/ai/cleanup-agent-backend.md`
- **Frontend Agent (B)**: `docs/ai/cleanup-agent-frontend.md`

Those files contain only the tasks and context needed for that agent. Do not edit the other agent’s spec; use TODO notes as described above when you need changes across the boundary.

