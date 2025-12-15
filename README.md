# Code Warrior: C Mastery

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)
[![Svelte](https://img.shields.io/badge/Svelte-5-ff3e00.svg)](https://svelte.dev/)

> A web-based RPG that teaches C programming through hands-on gameplay.

## What is Code Warrior?

Learn C by playing! Write real C code to solve puzzles, explore memory-themed worlds (heap → "Memory Marsh", stack → "Stack Spire"), and progress from `printf` to pointer arithmetic.

## Quick Start

### Prerequisites

- **Rust 2021+** with `cargo`
- **Node.js 18+** with `npm`
- **GCC** for C code execution
- **PostgreSQL** (local or [Neon](https://neon.tech) cloud)

### Run Locally

```bash
# Clone the repository
git clone <repository-url>
cd C-warrior

# Install frontend dependencies
cd src-ui
npm install

# Start frontend (in one terminal)
npm run dev -- --open

# Start API (in another terminal)
cd src-api
cargo run
```

## Project Structure

```
src/          → Shared Rust game logic
src-api/      → Axum HTTP backend
src-ui/       → Svelte 5 frontend
docs/         → Full documentation
tools/        → MCP servers & utilities
```

## Documentation

| Topic | Link |
|-------|------|
| Architecture | [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) |
| Game Design | [docs/GAME_DESIGN.md](docs/GAME_DESIGN.md) |
| Curriculum | [docs/CURRICULUM.md](docs/CURRICULUM.md) |
| Flow & Logic | [docs/logic-mindmap.md](docs/logic-mindmap.md) |

**For AI Agents**: See [AGENTS.md](AGENTS.md) and [tools/README.md](tools/README.md).

## Contributing

1. Read [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
2. Check [docs/CURRICULUM.md](docs/CURRICULUM.md) for content gaps
3. Follow [AGENTS.md](AGENTS.md) guidelines

## License

MIT - see [LICENSE](LICENSE)
