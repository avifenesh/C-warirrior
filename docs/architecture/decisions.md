# Technical Decisions Log

## Overview

This document records significant technical decisions made during Code Warrior development, including rationale, alternatives considered, and consequences.

---

## ADR-001: Backend-Authoritative Architecture

**Date**: 2024-01-01
**Status**: Accepted

### Context

Need to choose architectural pattern for game state management in an Axum-backed web app with Svelte frontend.

### Decision

Implement **backend-authoritative pattern** where Rust backend is the single source of truth for all game state. Svelte is purely a visualization layer.

### Rationale

1. **Security**: C code execution must be sandboxed in Rust, not in browser
2. **Educational Accuracy**: Game mechanics must exactly match C semantics, requiring systems-level control
3. **Performance**: Complex game logic (ECS, pathfinding) runs faster in Rust
4. **Consistency**: Single source of truth prevents desynchronization bugs
5. **AI Development**: Clear separation helps AI agents understand boundaries

### Alternatives Considered

| Alternative | Rejected Because |
|-------------|------------------|
| **Client-side game logic** | Cannot safely execute C code in browser; performance concerns |
| **Shared state (Redux-like)** | Too complex to synchronize; prone to race conditions |
| **Traditional client-server** | Unnecessary round-trips; prefer shared logic via WASM |

### Consequences

**Positive**:
- Clear responsibility boundaries
- Enhanced security
- Better performance for game logic
- AI agents can be constrained effectively

**Negative**:
- More IPC communication required
- Learning curve for developers used to frontend state management
- Latency between user input and visual feedback (mitigated with interpolation)

---

## ADR-002: Svelte 5 Runes Over Stores

**Date**: 2024-01-02
**Status**: Accepted

### Context

Need to choose reactive state management approach in Svelte frontend.

### Decision

Use **Svelte 5 Runes** ($state, $derived, $effect) exclusively. Prohibit use of Svelte stores.

### Rationale

1. **Performance**: Fine-grained reactivity reduces unnecessary re-renders
2. **Simplicity**: Runes have simpler mental model than stores
3. **Modern Standard**: Svelte 5 is the future of Svelte
4. **Game-Friendly**: High-frequency updates (60 FPS) work well with Runes
5. **AI Training**: Most AI models trained on Svelte 3/4; explicit constraints needed

### Alternatives Considered

| Alternative | Rejected Because |
|-------------|------------------|
| **Svelte stores** | Older pattern; more boilerplate; AI agents default to this |
| **Context API** | Too verbose for frequent state updates |
| **External state library** | Unnecessary dependency; Runes sufficient |

### Consequences

**Positive**:
- Better performance for high-frequency updates
- Less boilerplate code
- Future-proof codebase

**Negative**:
- Developers must unlearn stores pattern
- AI agents require explicit instructions to use Runes
- Smaller community knowledge base (for now)

---

## ADR-003: Wave Function Collapse for Procedural Generation

**Date**: 2024-01-03
**Status**: Accepted

### Context

Need to generate Memory Marsh maps that visually represent heap memory allocation patterns.

### Decision

Use **Wave Function Collapse (WFC)** algorithm for procedural map generation.

### Rationale

1. **Constraint-Based**: WFC enforces adjacency rules that match memory allocation patterns
2. **Educational**: Can encode rules like "allocated memory must be contiguous"
3. **Quality**: Produces more structured results than noise-based approaches
4. **Metaphor Accuracy**: WFC's "collapse" mirrors memory allocation decisions

### Alternatives Considered

| Alternative | Rejected Because |
|-------------|------------------|
| **Perlin Noise** | Too organic; doesn't encode memory semantics |
| **Drunkard's Walk** | Acceptable for simple caves; not structured enough |
| **Hand-crafted maps** | Not scalable; loses dynamic allocation metaphor |
| **BSP Trees** | Too rigid; better for traditional dungeons |

### Consequences

**Positive**:
- Maps accurately represent memory structure
- Can visualize fragmentation naturally
- Replayable levels with different layouts

**Negative**:
- More complex to implement than simpler algorithms
- Performance cost (mitigated by generating once, caching)
- Requires careful rule design

---

## ADR-004: Seccomp-BPF Sandbox for C Execution

**Date**: 2024-01-04
**Status**: Accepted (Updated 2024-12)

### Context

Need to safely execute untrusted C code submitted by players in containerized environments (Railway, Docker) where namespace-based isolation (bubblewrap) fails.

### Decision

Use **seccomp-BPF syscall filtering** as primary sandbox, with bubblewrap as fallback.

**Sandbox priority:**
1. **seccomp-bpf** (Linux, works in containers without privileges)
2. **bubblewrap** (Linux, requires namespace support)
3. **Fail hard** (panic if no sandbox available in production)

### Rationale

1. **Container-Compatible**: seccomp only needs `PR_SET_NO_NEW_PRIVS`, no namespace privileges
2. **Threading Support**: Allows pthreads (clone with CLONE_THREAD, futex)
3. **Security**: Blocks execve, fork, networking at kernel level
4. **Fail-Safe**: Refuses to start without sandbox (no silent fallback)

### Implementation

```rust
// Syscall whitelist (allow)
read, write, close, mmap, mprotect, brk,
clone (CLONE_THREAD only), futex,  // Threading
exit, exit_group, getpid, gettid

// Blocked (kill process)
execve, fork, vfork,                // No shell/processes
socket, connect, bind, accept,       // No networking
ptrace                               // No debugging
```

### Environment Variables

| Variable | Value | Effect |
|----------|-------|--------|
| `ALLOW_INSECURE_SANDBOX` | `1` | Enable fallback (dev only) |
| `ALLOW_INSECURE_SANDBOX` | unset | Panic if no sandbox |

### Alternatives Considered

| Alternative | Rejected Because |
|-------------|------------------|
| **bubblewrap only** | Fails in containers: "Creating new namespace failed: Operation not permitted" |
| **WASM (wasi)** | Threading support experimental; not production-ready |
| **Docker-in-Docker** | Requires privileged mode; complex setup |
| **External service (Judge0)** | Added latency; external dependency |

### Consequences

**Positive**:
- Works in Railway/Docker without special permissions
- Supports threading for advanced C lessons
- Explicit security model (fail-safe)
- Real C compilation with GCC

**Negative**:
- Linux-only (macOS dev requires ALLOW_INSECURE_SANDBOX=1)
- Must maintain syscall whitelist as C lessons evolve

---

## ADR-005: PostgreSQL + SQLx for Persistence

**Date**: 2024-01-05
**Status**: Accepted (Updated)

### Context

Need database solution for save states and progress tracking, with cloud deployment support.

### Decision

Use **PostgreSQL with SQLx** (Neon for production hosting).

### Rationale

1. **Cloud-Ready**: Works with Neon serverless Postgres
2. **Type Safety**: SQLx provides compile-time query checking
3. **Transactions**: ACID guarantees prevent save corruption
4. **Async**: Native async/await support with Tokio
5. **Rust Ecosystem**: Well-supported, mature tooling

### Alternatives Considered

| Alternative | Rejected Because |
|-------------|------------------|
| **SQLite + Diesel** | Less cloud-friendly; Diesel has more complex async story |
| **JSON files** | No transactions; prone to corruption; manual schema management |
| **MongoDB** | Overkill for this use case; more complex setup |
| **In-memory only** | Loses progress on crash |

### Consequences

**Positive**:
- Type-safe queries
- Reliable persistence
- Cloud deployment ready
- Async-first design

**Negative**:
- Requires Postgres instance (Neon free tier works)
- Connection pooling considerations for serverless

---

## ADR-006: ts-rs for Type Generation

**Date**: 2024-01-06
**Status**: Accepted

### Context

Need to synchronize types between Rust backend and TypeScript frontend.

### Decision

Use **ts-rs to generate TypeScript types from Rust structs** automatically.

### Rationale

1. **Single Source of Truth**: Rust types are authoritative
2. **Compile-Time**: Type mismatches caught early
3. **Zero Maintenance**: Types update automatically
4. **Prevents Drift**: Cannot forget to update frontend types

### Alternatives Considered

| Alternative | Rejected Because |
|-------------|------------------|
| **Manual TypeScript types** | Prone to drift; error-prone; duplicate work |
| **JSON Schema** | Extra layer of indirection |
| **GraphQL** | Overkill for IPC communication |
| **OpenAPI** | Designed for HTTP APIs, not IPC |

### Consequences

**Positive**:
- Type safety across IPC boundary
- Automatic updates
- Reduces bugs

**Negative**:
- Adds build step
- Requires Rust rebuild to update types
- Not all Rust types map cleanly to TypeScript

---

## ADR-007: 20 TPS Game Loop, 60 FPS Rendering

**Date**: 2024-01-07
**Status**: Accepted

### Context

Need to choose tick rate for game logic and rendering frame rate.

### Decision

**Game logic: 20 TPS (ticks per second)**
**Rendering: 60 FPS (frames per second)**

### Rationale

1. **Separation**: Decouples game logic from rendering
2. **Performance**: 20 TPS sufficient for turn-based/puzzle gameplay
3. **Network-Ready**: 20 TPS standard for multiplayer games (future expansion)
4. **Smooth Rendering**: Frontend interpolates for 60 FPS visuals
5. **Bandwidth**: Fewer IPC events (20 vs 60 per second)

### Alternatives Considered

| Alternative | Rejected Because |
|-------------|------------------|
| **60 TPS** | Unnecessary CPU usage; more IPC overhead |
| **Variable tick rate** | Inconsistent behavior; harder to debug |
| **Render-locked** | Ties logic to frame rate (bad practice) |

### Consequences

**Positive**:
- Predictable game logic timing
- Reduced IPC traffic
- Smooth visuals via interpolation

**Negative**:
- Need to implement interpolation in frontend
- Slight complexity in state synchronization

---

## ADR-008: 80/20 Active Learning Rule

**Date**: 2024-01-08
**Status**: Accepted

### Context

Need to balance education and gameplay engagement.

### Decision

Enforce **80% coding, 20% reading/learning** rule for all levels.

### Rationale

1. **Active Learning**: Research shows hands-on practice is most effective
2. **Engagement**: Players stay engaged by doing, not reading
3. **Retention**: Writing code builds muscle memory
4. **Flow State**: Coding is intrinsically rewarding

### Alternatives Considered

| Alternative | Rejected Because |
|-------------|------------------|
| **50/50 split** | Too much passive learning; less engaging |
| **Pure sandbox** | No structure; players get lost |
| **Tutorial-heavy** | Players skip tutorials; low retention |

### Consequences

**Positive**:
- Higher engagement
- Better learning outcomes
- Memorable experience

**Negative**:
- Must carefully design code challenges
- Players may get frustrated without proper hints
- Requires strong hint system

---

## ADR-009: Pointer = Grappling Hook Metaphor

**Date**: 2024-01-09
**Status**: Accepted

### Context

Need intuitive visual metaphor for C pointers.

### Decision

Represent pointers as **grappling hooks** (visual lines connecting player to objects).

### Rationale

1. **Indirection**: Hook shows "pointing to" relationship
2. **Action**: Pulling hook = dereferencing
3. **Spatial**: Makes abstract concept physical
4. **Gamification**: Useful tool for traversal puzzles
5. **Memorable**: Unique metaphor aids retention

### Alternatives Considered

| Alternative | Rejected Because |
|-------------|------------------|
| **Arrow signs** | Too abstract; not interactive |
| **Teleportation** | Misses the "reference" concept |
| **Magic wand** | Doesn't convey indirection clearly |

### Consequences

**Positive**:
- Intuitive understanding of pointers
- Enables creative puzzles (reach distant objects)
- Visually clear

**Negative**:
- Metaphor breaks for pointer arithmetic (teleportation-like)
- Need to explain NULL = broken hook

---

## ADR-010: Memory Leak = Enemy Spawn

**Date**: 2024-01-10
**Status**: Accepted

### Context

Need consequence for memory leaks that feels game-like but teaches the concept.

### Decision

**Spawn "Slime Monster" enemy on leaked memory tiles**. Monster slowly drains player health until leak is freed.

### Rationale

1. **Consequence**: Makes abstract problem tangible
2. **Progressive**: Leak damage increases over time (urgency)
3. **Reversible**: Calling `free()` despawns monster
4. **Memorable**: Players remember "leaks = bad" viscerally
5. **Gamification**: Adds threat without instant death

### Alternatives Considered

| Alternative | Rejected Because |
|-------------|------------------|
| **Instant death** | Too punishing; discourages experimentation |
| **Score penalty** | Abstract; doesn't feel meaningful |
| **Visual warning only** | Easy to ignore; no stakes |
| **Performance degradation** | Confusing; not obvious |

### Consequences

**Positive**:
- Clear cause-and-effect
- Encourages memory cleanup
- Adds challenge to gameplay

**Negative**:
- Could be stressful for some players
- Need to balance difficulty (drain rate)

---

## Future Decisions Needed

### Under Consideration

1. **Multiplayer Architecture**: Peer-to-peer vs server-authoritative?
2. **Asset Pipeline**: Custom tool vs manual workflow?
3. **Level Editor**: In-game vs external tool (Tiled)?
4. **Audio System**: Which Rust audio library to use?
5. **Localization**: i18n strategy for international release?
6. **Platform Distribution**: Steam vs itch.io vs self-hosted?

---

## Decision Review Schedule

- **Monthly**: Review recent decisions for effectiveness
- **Quarterly**: Assess if any decisions should be reversed
- **Per-Phase**: Major decisions reviewed at phase completion

---

**Template for New Decisions**:

```markdown
## ADR-XXX: [Short Title]

**Date**: YYYY-MM-DD
**Status**: Proposed | Accepted | Deprecated | Superseded

### Context
[Describe the problem/question]

### Decision
[State the decision clearly]

### Rationale
[Explain why this decision was made]

### Alternatives Considered
[List and explain rejected options]

### Consequences
**Positive**: [List benefits]
**Negative**: [List drawbacks/costs]
```
