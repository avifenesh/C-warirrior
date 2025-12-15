# Code Warrior: C Mastery

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)
[![Svelte](https://img.shields.io/badge/Svelte-5-ff3e00.svg)](https://svelte.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5-blue.svg)](https://www.typescriptlang.org/)

> 🎮 An immersive web-based RPG that teaches C programming from zero to hero through hands-on gameplay.

**[🚀 Play Now](https://code-warrior-seven.vercel.app)** | **[📖 Documentation](docs/)** | **[🤝 Contributing](#contributing)**

## Overview

Code Warrior transforms learning C into an interactive adventure where programming concepts become tangible game mechanics. Players explore procedurally-generated worlds that visualize computer memory, solve coding puzzles to progress, and master low-level programming through 80% active coding and 20% guided instruction.

## Key Features

- **Memory as World**: The heap becomes "Memory Marsh," the stack becomes "Stack Spire"
- **Pointers as Tools**: Use pointer operations as "grappling hooks" to navigate and interact
- **Code-Driven Gameplay**: Write real C code to cast spells, unlock doors, and defeat challenges
- **Visual Debugging**: See your code execution as in-game visualization
- **Progressive Curriculum**: From `printf` to pointer arithmetic, structured learning path

## Technology Stack

| Component | Technology | Purpose |
|-----------|-----------|---------|
| **Backend** | Rust (Axum HTTP API) | Game engine, C runtime, state management |
| **Frontend** | Svelte 5 (Runes) | Reactive UI and visualization |
| **Database** | PostgreSQL + SQLx (Neon/Railway) | Persistent save states and sessions |
| **Architecture** | Backend-Authoritative | Rust as single source of truth |

## Architecture Philosophy

Code Warrior follows a **backend-authoritative** model where:
- **Rust handles**: Game logic, physics, C code execution, state management
- **Svelte handles**: Visualization, input capture, UI rendering
- **Communication**: HTTP/Fetch (web) with embedded WASM for fast client-side state projection

This separation ensures security, performance, and clear responsibilities for both human developers and AI coding agents.

## Documentation Structure

### Core Documentation
- **[Logic Mind Map](docs/logic-mindmap.md)** - Source of truth for end-to-end flow and file locations
- **[ARCHITECTURE.md](docs/ARCHITECTURE.md)** - Technical architecture and design patterns
- **[GAME_DESIGN.md](docs/GAME_DESIGN.md)** - RPG mechanics and C concept mappings
- **[CURRICULUM.md](docs/CURRICULUM.md)** - Educational framework and learning progression
- **[IMPLEMENTATION.md](docs/IMPLEMENTATION.md)** - Code examples and implementation guides

### Development Tools
- **[AGENTS.md](AGENTS.md)** - AI agent guidelines (all agents)
- **[CLAUDE.md](CLAUDE.md)** - Claude Code specific guidelines
- **[docs/core/TESTING.md](docs/core/TESTING.md)** - Testing protocol
- **[docs/ai/gemini-usage.md](docs/ai/gemini-usage.md)** - Gemini CLI guide
- **[tools/](tools/)** - MCP servers and utility scripts

### Reference
- **[docs/DECISIONS.md](docs/DECISIONS.md)** - Technical decision log

## Quick Start

### Prerequisites
- Rust 2021+ with `cargo`
- Node.js 18+ with `npm`
- GCC compiler for C code execution
- PostgreSQL (local instance or Neon cloud)

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd C-warrior

# Install frontend dependencies
cd src-ui
npm install
cd ..

# Run frontend (web dev)
cd src-ui
npm run dev -- --open

# Run API locally
cd ../src-api
cargo run
```

**Note**: Run the API server and frontend dev server in separate terminals for local development.

### For AI-Assisted Development

1. **Configure MCP Tools**: See [tools/README.md](tools/README.md) for MCP server setup
2. **Agent Rules**: Follow the guidance in [AGENTS.md](AGENTS.md)
3. **Review Architecture**: Read [ARCHITECTURE.md](docs/ARCHITECTURE.md) to understand the system boundaries

## Project Goals

### Educational Objectives
- Teach C programming from absolute beginner to advanced
- Make memory management intuitive through spatial metaphors
- Build confidence through immediate visual feedback
- Encourage experimentation in a safe, gamified environment

### Technical Objectives
- Demonstrate modern Rust/Svelte integration
- Implement secure C code sandboxing
- Create reusable educational game engine patterns
- Optimize for AI-assisted development workflows

## Development Philosophy

### The 80/20 Rule
Players spend **80% of time writing code** and **20% reading/learning**. Every level must require actual C programming to progress.

### Backend-Authoritative Pattern
All game state lives in Rust. Svelte never makes decisions, only visualizes. This prevents security issues and maintains consistency.

### Progressive Disclosure
Start simple (`printf`, variables) and gradually introduce complexity (pointers, memory management). Each concept builds on the previous.

### Test-First Design
Write the C challenge first, create verification tests, then implement game rewards. This ensures educational content is technically accurate.

## Contributing

We welcome contributions! Please:
1. Read the [ARCHITECTURE.md](docs/ARCHITECTURE.md) to understand the system
2. Check the curriculum in [CURRICULUM.md](docs/CURRICULUM.md) for content gaps
3. Follow the AI/agent guidelines in [AGENTS.md](AGENTS.md)
4. Follow the testing guidelines in [IMPLEMENTATION.md](docs/IMPLEMENTATION.md)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

For questions about the architecture or implementation, refer to:
- [Architecture Guide](docs/ARCHITECTURE.md) for system design
- [Implementation Guide](docs/IMPLEMENTATION.md) for code examples
- [Game Design Document](docs/GAME_DESIGN.md) for metaphor mappings

---

**Built with Rust, Svelte, and a love for systems programming education.**
