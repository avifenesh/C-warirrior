# Agent B: Strings Content

## Goal
Create 5 levels teaching string manipulation (L31-L35).

## Status: ⏳ PENDING

## Files to Create
- `src/assets/maps/L31_measuring_spell.json`
- `src/assets/maps/L32_clone_scroll.json`
- `src/assets/maps/L33_combining_enchantments.json`
- `src/assets/maps/L34_password_verification.json`
- `src/assets/maps/L35_inscription_forge.json`
- Update `src/assets/levels.json` with L31-L35 entries

## Level Specifications

### L31: Measuring the Spell (strlen)
**Concept**: Getting string length
**Metaphor**: Measuring the power of a spell by its incantation length
**Code Challenge**:
```c
#include <stdio.h>
#include <string.h>

int main() {
    char spell[] = "Abracadabra";
    int length = strlen(spell);
    printf("Spell length: %d\n", length);
    return 0;
}
```
**Expected Output**: `Spell length: 11\n`
**XP**: 700

### L32: The Clone Scroll (strcpy)
**Concept**: Copying strings
**Metaphor**: Creating a copy of a magical scroll
**Code Challenge**:
```c
#include <stdio.h>
#include <string.h>

int main() {
    char original[] = "Fire Ball";
    char copy[20];

    strcpy(copy, original);
    printf("Original: %s\n", original);
    printf("Copy: %s\n", copy);
    return 0;
}
```
**Expected Output**: `Original: Fire Ball\nCopy: Fire Ball\n`
**XP**: 725

### L33: Combining Enchantments (strcat)
**Concept**: Concatenating strings
**Metaphor**: Combining two enchantments into one powerful spell
**Code Challenge**:
```c
#include <stdio.h>
#include <string.h>

int main() {
    char spell[50] = "Fire ";
    char modifier[] = "Storm";

    strcat(spell, modifier);
    printf("Combined spell: %s\n", spell);
    return 0;
}
```
**Expected Output**: `Combined spell: Fire Storm\n`
**XP**: 750

### L34: Password Verification (strcmp)
**Concept**: Comparing strings
**Metaphor**: Verifying the password to enter a secret chamber
**Code Challenge**:
```c
#include <stdio.h>
#include <string.h>

int main() {
    char password[] = "OpenSesame";
    char attempt[] = "OpenSesame";

    if (strcmp(password, attempt) == 0) {
        printf("Access granted!\n");
    } else {
        printf("Access denied!\n");
    }
    return 0;
}
```
**Expected Output**: `Access granted!\n`
**XP**: 775

### L35: The Inscription Forge (sprintf)
**Concept**: Formatting strings
**Metaphor**: Forging custom inscriptions for weapons
**Code Challenge**:
```c
#include <stdio.h>

int main() {
    char inscription[100];
    char weapon[] = "Sword";
    int damage = 50;
    char element[] = "Fire";

    sprintf(inscription, "%s of %s (+%d damage)", weapon, element, damage);
    printf("Inscription: %s\n", inscription);
    return 0;
}
```
**Expected Output**: `Inscription: Sword of Fire (+50 damage)\n`
**XP**: 800

## Tasks
1. [ ] Create L31 map: strlen ("Measuring the Spell")
2. [ ] Create L32 map: strcpy ("Clone Scroll")
3. [ ] Create L33 map: strcat ("Combining Enchantments")
4. [ ] Create L34 map: strcmp ("Password Verification")
5. [ ] Create L35 map: sprintf ("Inscription Forge")
6. [ ] Add L31-L35 entries to levels.json
7. [ ] Validate all C puzzles with compile_and_run_c
8. [ ] Validate JSON: `python -m json.tool`

## Map Reference
See `src/assets/maps/L21_summon_land.json` for format.

Each map should include:
- spawn point
- terminal (for code challenge)
- NPC with concept hint
- door (locked until puzzle solved)

## Notes
- string.h must be included for string functions
- Always ensure destination buffers are large enough
- strcmp returns 0 for equal strings (not 1!)
