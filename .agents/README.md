# Multi-Agent Coordination System

This directory contains the coordination infrastructure for running 6 parallel AI agents on the Code Warrior project.

---

## Quick Start

### 1. Read the Coordination File
All agents MUST read [COORDINATION.md](./COORDINATION.md) before starting.

### 2. Launch Agents
Use the prompts in [prompts/](./prompts/) to launch each agent.

### 3. Monitor Progress
Check COORDINATION.md for:
- File lock status
- Agent completion checklists
- Communication log

---

## Directory Structure

```
.agents/
├── README.md              # This file
├── COORDINATION.md        # Lock table & status (AGENTS READ THIS)
└── prompts/
    ├── AGENT_1_DATABASE.md
    ├── AGENT_2_FRONTEND.md
    ├── AGENT_3_MAPS_6_10.md
    ├── AGENT_4_MAPS_11_15.md
    ├── AGENT_5_LEVELS.md
    └── AGENT_6_INTEGRATION.md
```

---

## Agent Overview

| Agent | Role | Parallel? | Dependencies |
|-------|------|-----------|--------------|
| 1 | Database Layer | ✅ Yes | None |
| 2 | Frontend Components | ✅ Yes | None |
| 3 | Maps L06-L10 | ✅ Yes | None |
| 4 | Maps L11-L15 | ✅ Yes | None |
| 5 | Level Content | ⚠️ Waits | Agents 3 & 4 |
| 6 | Integration | ⚠️ Waits | ALL agents |

---

## Workflow

```
Phase 1: Parallel Execution
┌─────────────────────────────────────────┐
│  Agent 1 ████████ Database Layer        │
│  Agent 2 ████████ Frontend Components   │
│  Agent 3 ████████ Maps L06-L10          │
│  Agent 4 ████████ Maps L11-L15          │
└─────────────────────────────────────────┘
              ↓
Phase 2: Content Creation (waits for maps)
┌─────────────────────────────────────────┐
│  Agent 5 ████████ Level Definitions     │
└─────────────────────────────────────────┘
              ↓
Phase 3: Integration (waits for all)
┌─────────────────────────────────────────┐
│  Agent 6 ████████ Wire & Test           │
└─────────────────────────────────────────┘
```

---

## File Locking Protocol

1. **Check** COORDINATION.md before touching any file
2. **Claim** by editing the lock table (Status: LOCKED, Locked By: Agent X)
3. **Work** on your claimed files only
4. **Release** when done (Status: DONE)
5. **Request** if locked by another (add to Waiting column)

---

## Conflict Resolution

- First to update COORDINATION.md wins the lock
- Stale locks (>30 min) can be claimed
- When in doubt, wait and re-check
- Agent 6 is the final integrator and arbiter

---

## Communication

Agents communicate via the Communication Log in COORDINATION.md:
```
[2024-01-15 10:30] [Agent 1]: Database module complete, ready for integration
[2024-01-15 10:45] [Agent 3]: Maps L06-L10 done, Agent 5 can start
```

---

## Launching Agents

### Claude Code (CLI)
```bash
# Launch Agent 1
claude --prompt "$(cat .agents/prompts/AGENT_1_DATABASE.md)"

# Launch multiple in parallel (different terminals)
claude --prompt "$(cat .agents/prompts/AGENT_2_FRONTEND.md)" &
claude --prompt "$(cat .agents/prompts/AGENT_3_MAPS_6_10.md)" &
```

### Factory/Cursor/Other
Copy the full contents of each prompt file and paste into the agent.

---

## Troubleshooting

### Agent trying to modify wrong file
→ Check COORDINATION.md - file may be locked by another agent

### Lock conflict
→ First agent to update COORDINATION.md wins

### Agent 5 starting too early
→ Verify Agents 3 & 4 marked maps as DONE first

### Integration failures
→ Agent 6 reports issues in Communication Log, relevant agent fixes

---

## Best Practices

1. **Always read COORDINATION.md first**
2. **Claim locks before writing**
3. **Release locks immediately when done**
4. **Use Communication Log for status updates**
5. **Don't touch other agents' domain files**
6. **Verify output before marking DONE**
