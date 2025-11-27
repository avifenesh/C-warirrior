# Multi-Agent Coordination Hub

**Current Round**: 3
**Total Levels**: 25 (targeting 35 after this round)

---

## Round 3 Agent Assignments

| Agent | Role | Status | Primary Mission |
|-------|------|--------|-----------------|
| **A** | File I/O Content | ⏳ PENDING | Create L26-L30 (fopen, fprintf, fscanf, fread/fwrite, fseek) |
| **B** | Strings Content | ⏳ PENDING | Create L31-L35 (strlen, strcpy, strcat, strcmp, sprintf) |
| **C** | System Integration | ⏳ PENDING | Wire DB handlers, frontend polish, save/load |

---

## File Lock Table

| File Path | Status | Owner | Notes |
|-----------|--------|-------|-------|
| **Maps L26-L30 (File I/O)** |
| `src/assets/maps/L26_scroll_archive.json` | ⏳ PENDING | A | fopen/fclose |
| `src/assets/maps/L27_scribes_quill.json` | ⏳ PENDING | A | fprintf |
| `src/assets/maps/L28_ancient_texts.json` | ⏳ PENDING | A | fscanf |
| `src/assets/maps/L29_binary_artifacts.json` | ⏳ PENDING | A | fread/fwrite |
| `src/assets/maps/L30_map_navigator.json` | ⏳ PENDING | A | fseek/ftell |
| **Maps L31-L35 (Strings)** |
| `src/assets/maps/L31_measuring_spell.json` | ⏳ PENDING | B | strlen |
| `src/assets/maps/L32_clone_scroll.json` | ⏳ PENDING | B | strcpy |
| `src/assets/maps/L33_combining_enchantments.json` | ⏳ PENDING | B | strcat |
| `src/assets/maps/L34_password_verification.json` | ⏳ PENDING | B | strcmp |
| `src/assets/maps/L35_inscription_forge.json` | ⏳ PENDING | B | sprintf |
| **Level Definitions** |
| `src/assets/levels.json` | 🔒 SHARED | A, B | A adds L26-30, B adds L31-35 (coordinate!) |
| **Backend** |
| `src-api/src/main.rs` | ⏳ PENDING | C | Wire handlers |
| `src-api/src/db/operations.rs` | ⏳ PENDING | C | DB operations |
| **Frontend** |
| `src-ui/src/lib/components/` | ⏳ PENDING | C | UI polish |

---

## Agent Tasks

### Agent A: File I/O Content
**Goal**: Create 5 levels teaching file operations

**Prompt File**: `.agents/prompts/AGENT_A_FILE_IO.md`

**Levels**:
| Level | Concept | Title | XP |
|-------|---------|-------|-----|
| L26 | fopen/fclose | The Scroll Archive | 575 |
| L27 | fprintf | The Scribe's Quill | 600 |
| L28 | fscanf | Reading Ancient Texts | 625 |
| L29 | fread/fwrite | Binary Artifacts | 650 |
| L30 | fseek/ftell | The Map Navigator | 675 |

**Tasks**:
1. [ ] Create 5 map files (L26-L30)
2. [ ] Add L26-L30 entries to levels.json
3. [ ] Validate C puzzles with compile_and_run_c
4. [ ] Validate JSON syntax

---

### Agent B: Strings Content
**Goal**: Create 5 levels teaching string manipulation

**Prompt File**: `.agents/prompts/AGENT_B_STRINGS.md`

**Levels**:
| Level | Concept | Title | XP |
|-------|---------|-------|-----|
| L31 | strlen | Measuring the Spell | 700 |
| L32 | strcpy | The Clone Scroll | 725 |
| L33 | strcat | Combining Enchantments | 750 |
| L34 | strcmp | Password Verification | 775 |
| L35 | sprintf | The Inscription Forge | 800 |

**Tasks**:
1. [ ] Create 5 map files (L31-L35)
2. [ ] Add L31-L35 entries to levels.json
3. [ ] Validate C puzzles with compile_and_run_c
4. [ ] Validate JSON syntax

---

### Agent C: System Integration
**Goal**: Complete backend wiring and frontend polish

**Prompt File**: `.agents/prompts/AGENT_C_SYSTEM.md`

**Tasks**:
1. [ ] Wire init_game handler to DB
2. [ ] Wire submit_code handler to DB
3. [ ] Wire save/load_progress handlers
4. [ ] Fix any frontend type errors
5. [ ] Add level select UI
6. [ ] Test E2E on production

---

## Dependencies

```
Agent A (File I/O L26-30) ─────┬──→ levels.json (coordinate with B)
                               │
Agent B (Strings L31-35) ──────┤
                               │
Agent C (System Integration) ──┴──→ Independent (backend/frontend)
```

**Coordination for levels.json**:
- Agent A adds L26-L30 FIRST
- Agent B adds L31-L35 AFTER Agent A completes
- Or: One agent adds all 10 entries

---

## Curriculum Progress

| Phase | Levels | Topics | Status |
|-------|--------|--------|--------|
| 1 | L01-L05 | Foundations | ✅ Complete |
| 2 | L06-L10 | Functions | ✅ Complete |
| 3 | L11-L15 | Pointers | ✅ Complete |
| 4 | L16-L25 | Structs + Memory | ✅ Complete |
| 5 | L26-L30 | File I/O | ⏳ Round 3 |
| 5 | L31-L35 | Strings | ⏳ Round 3 |

---

## Communication Log

```
=== ROUND 1 COMPLETE ===
[2025-11-26] Initial setup complete
  - L01-L15 levels and maps created
  - Frontend and backend builds passing

=== ROUND 2 COMPLETE ===
[2025-11-27] Content expansion
  - L16-L25 levels and maps created (Structs + Memory)
  - DB retry logic and Neon pool optimization
  - E2E tests passing on production
  - Commit: 45d4a5d

=== ROUND 3 STARTING ===
[2025-11-27] Planning:
  - Agent A: File I/O content (L26-L30)
  - Agent B: Strings content (L31-L35)
  - Agent C: System integration (DB wiring, UI polish)
```

---

## Quick Commands

```bash
# Backend check
cd src-api && cargo check

# Frontend check
cd src-ui && npm run check

# Validate JSON
python -m json.tool < src/assets/levels.json

# Test C puzzle
# Use MCP: compile_and_run_c(source_code, input_data)

# E2E test
cd ~/.claude/plugins/marketplaces/playwright-skill/skills/playwright-skill && \
  node run.js /tmp/playwright-test-codewarrior-e2e.js
```

---

## Production URLs

| Service | URL |
|---------|-----|
| Frontend | https://code-warrior-seven.vercel.app |
| API | https://code-warrior-api-production.up.railway.app |
| Database | Neon PostgreSQL |
