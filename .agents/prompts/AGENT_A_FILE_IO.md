# Agent A: File I/O Content

## Goal
Create 5 levels teaching file operations (L26-L30).

## Status: ⏳ PENDING

## Files to Create
- `src/assets/maps/L26_scroll_archive.json`
- `src/assets/maps/L27_scribes_quill.json`
- `src/assets/maps/L28_ancient_texts.json`
- `src/assets/maps/L29_binary_artifacts.json`
- `src/assets/maps/L30_map_navigator.json`
- Update `src/assets/levels.json` with L26-L30 entries

## Level Specifications

### L26: The Scroll Archive (fopen/fclose)
**Concept**: Opening and closing files
**Metaphor**: Ancient archive where scrolls must be properly opened and closed
**Code Challenge**:
```c
#include <stdio.h>

int main() {
    FILE *scroll = fopen("archive.txt", "w");
    if (scroll == NULL) {
        printf("Failed to open scroll!\n");
        return 1;
    }
    printf("Scroll opened successfully!\n");
    fclose(scroll);
    printf("Scroll closed properly.\n");
    return 0;
}
```
**Expected Output**: `Scroll opened successfully!\nScroll closed properly.\n`
**XP**: 575

### L27: The Scribe's Quill (fprintf)
**Concept**: Writing to files with fprintf
**Metaphor**: Magical quill that writes to scrolls
**Code Challenge**:
```c
#include <stdio.h>

int main() {
    FILE *scroll = fopen("message.txt", "w");
    if (scroll == NULL) return 1;

    fprintf(scroll, "The warrior has arrived!\n");
    fprintf(scroll, "Gold collected: %d\n", 500);

    fclose(scroll);
    printf("Message written to scroll.\n");
    return 0;
}
```
**Expected Output**: `Message written to scroll.\n`
**XP**: 600

### L28: Reading Ancient Texts (fscanf)
**Concept**: Reading from files with fscanf
**Metaphor**: Deciphering ancient scrolls to learn secrets
**Code Challenge**:
```c
#include <stdio.h>

int main() {
    FILE *scroll = fopen("secrets.txt", "r");
    if (scroll == NULL) {
        printf("Scroll not found!\n");
        return 1;
    }

    char word[50];
    int number;
    fscanf(scroll, "%s %d", word, &number);
    printf("Secret: %s %d\n", word, number);

    fclose(scroll);
    return 0;
}
```
**Expected Output**: `Secret: TREASURE 1000\n` (with pre-created file)
**XP**: 625

### L29: Binary Artifacts (fread/fwrite)
**Concept**: Binary file operations
**Metaphor**: Storing magical artifacts in binary format
**Code Challenge**:
```c
#include <stdio.h>

struct Artifact {
    int power;
    int durability;
};

int main() {
    struct Artifact sword = {100, 50};

    FILE *vault = fopen("artifact.bin", "wb");
    fwrite(&sword, sizeof(struct Artifact), 1, vault);
    fclose(vault);

    struct Artifact loaded = {0, 0};
    vault = fopen("artifact.bin", "rb");
    fread(&loaded, sizeof(struct Artifact), 1, vault);
    fclose(vault);

    printf("Loaded: power=%d, durability=%d\n", loaded.power, loaded.durability);
    return 0;
}
```
**Expected Output**: `Loaded: power=100, durability=50\n`
**XP**: 650

### L30: The Map Navigator (fseek/ftell)
**Concept**: File positioning
**Metaphor**: Navigating through a long map scroll
**Code Challenge**:
```c
#include <stdio.h>

int main() {
    FILE *map = fopen("map.txt", "w+");
    fprintf(map, "ABCDEFGHIJ");

    fseek(map, 5, SEEK_SET);  // Move to position 5
    long pos = ftell(map);
    printf("Position: %ld\n", pos);

    char c;
    fscanf(map, "%c", &c);
    printf("Character at position 5: %c\n", c);

    fclose(map);
    return 0;
}
```
**Expected Output**: `Position: 5\nCharacter at position 5: F\n`
**XP**: 675

## Tasks
1. [ ] Create L26 map: fopen/fclose ("Scroll Archive")
2. [ ] Create L27 map: fprintf ("Scribe's Quill")
3. [ ] Create L28 map: fscanf ("Ancient Texts")
4. [ ] Create L29 map: fread/fwrite ("Binary Artifacts")
5. [ ] Create L30 map: fseek/ftell ("Map Navigator")
6. [ ] Add L26-L30 entries to levels.json
7. [ ] Validate all C puzzles with compile_and_run_c
8. [ ] Validate JSON: `python -m json.tool`

## Map Reference
See `src/assets/maps/L21_summon_land.json` for format.

Each map should include:
- spawn point
- terminal (for code challenge)
- NPC with concept hint
- door (locked until puzzle solved)
