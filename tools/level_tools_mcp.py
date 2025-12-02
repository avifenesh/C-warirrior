"""
Level Tools MCP Server for Code Warrior.
Level management, validation, and analysis.
"""

from fastmcp import FastMCP
import json
import os
from typing import Optional

mcp = FastMCP("Code-Warrior-Levels")

PROJECT_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
LEVELS_FILE = os.path.join(PROJECT_ROOT, "src", "assets", "levels.json")


def load_levels() -> dict:
    """Load levels.json data. Handles both list and dict formats."""
    try:
        with open(LEVELS_FILE, 'r') as f:
            data = json.load(f)
            # Handle both formats: list or {"levels": [...]}
            if isinstance(data, list):
                return {"levels": data}
            return data
    except Exception as e:
        return {"error": str(e), "levels": []}


def save_levels(data: dict):
    """Save levels.json data."""
    with open(LEVELS_FILE, 'w') as f:
        json.dump(data, f, indent=2)


def _get_level_internal(level_id: str) -> dict:
    """Internal helper to get a level by ID."""
    data = load_levels()

    if "error" in data:
        return {"error": data["error"]}

    for level in data.get("levels", []):
        if level.get("id") == level_id:
            return level

    return {"error": f"Level '{level_id}' not found"}


@mcp.tool
def get_level(level_id: str) -> dict:
    """
    Get full level definition from levels.json.

    Args:
        level_id: Level ID (e.g., "level_1", "level_15")

    Returns:
        Full level object or error
    """
    return _get_level_internal(level_id)


@mcp.tool
def list_all_levels() -> dict:
    """
    List all levels with summary info.

    Returns:
        {
            "count": int,
            "levels": [{id, title, concept, quest_count}]
        }
    """
    data = load_levels()

    if "error" in data:
        return {"error": data["error"], "count": 0, "levels": []}

    levels = []
    for level in data.get("levels", []):
        levels.append({
            "id": level.get("id"),
            "title": level.get("title"),
            "concept": level.get("concept", ""),
            "quest_count": len(level.get("quests", []))
        })

    return {
        "count": len(levels),
        "levels": levels
    }


@mcp.tool
def validate_level(level_data: dict) -> dict:
    """
    Validate level structure against schema.

    Args:
        level_data: Level object to validate

    Returns:
        {
            "valid": bool,
            "errors": [str],
            "warnings": [str]
        }
    """
    errors = []
    warnings = []

    # Required fields
    required = ["id", "title", "quests"]
    for field in required:
        if field not in level_data:
            errors.append(f"Missing required field: {field}")

    # Validate ID format
    level_id = level_data.get("id", "")
    if level_id and not level_id.startswith("level_"):
        warnings.append(f"Level ID '{level_id}' should start with 'level_'")

    # Validate quests
    quests = level_data.get("quests", [])
    if not quests:
        errors.append("Level must have at least one quest")

    for i, quest in enumerate(quests):
        quest_id = quest.get("id", f"quest_{i}")

        # Required quest fields
        quest_required = ["id", "title", "challenge"]
        for field in quest_required:
            if field not in quest:
                errors.append(f"Quest '{quest_id}' missing required field: {field}")

        # Validate challenge
        challenge = quest.get("challenge", {})
        if challenge:
            if "starter_code" not in challenge:
                warnings.append(f"Quest '{quest_id}' has no starter_code")
            if "test_cases" not in challenge and "validation" not in challenge:
                errors.append(f"Quest '{quest_id}' needs test_cases or validation")

        # Check for hints
        if "hints" not in quest:
            warnings.append(f"Quest '{quest_id}' has no hints (consider adding for UX)")

    return {
        "valid": len(errors) == 0,
        "errors": errors,
        "warnings": warnings
    }


@mcp.tool
def get_level_template(concept: str) -> dict:
    """
    Get a template for creating a new level based on C concept.

    Args:
        concept: C concept (e.g., "pointers", "arrays", "structs")

    Returns:
        Template level object
    """
    # Base template
    template = {
        "id": "level_XX",
        "title": f"{concept.title()} Challenge",
        "concept": concept,
        "description": f"Learn {concept} through hands-on coding challenges.",
        "quests": [
            {
                "id": "quest_1",
                "title": f"Introduction to {concept.title()}",
                "description": f"Your first challenge with {concept}.",
                "concept": concept,
                "challenge": {
                    "type": "code",
                    "language": "c",
                    "starter_code": f"// TODO: Implement {concept} solution\n#include <stdio.h>\n\nint main() {{\n    // Your code here\n    return 0;\n}}",
                    "test_cases": [
                        {"input": "", "expected": ""}
                    ]
                },
                "hints": [
                    f"Hint 1: Remember the basics of {concept}",
                    "Hint 2: Think about the approach step by step",
                    "Hint 3: Almost there! Check your syntax"
                ],
        "reward": {
            "xp": 100
        }
            }
        ],
        "map": {
            "tileset": "dungeon",
            "spawn": {"x": 2, "y": 2}
        }
    }

    # Customize based on concept
    concept_customizations = {
        "pointers": {
            "starter_code": "#include <stdio.h>\n\nvoid swap(int *a, int *b) {\n    // Swap the values using pointers\n}\n\nint main() {\n    int x = 5, y = 10;\n    swap(&x, &y);\n    printf(\"%d %d\\n\", x, y);\n    return 0;\n}"
        },
        "arrays": {
            "starter_code": "#include <stdio.h>\n\nint sum_array(int arr[], int n) {\n    // Return the sum of array elements\n}\n\nint main() {\n    int arr[] = {1, 2, 3, 4, 5};\n    printf(\"%d\\n\", sum_array(arr, 5));\n    return 0;\n}"
        },
        "malloc": {
            "starter_code": "#include <stdio.h>\n#include <stdlib.h>\n\nint* create_array(int n) {\n    // Allocate and return an array of n integers\n}\n\nint main() {\n    int *arr = create_array(5);\n    // Use array\n    free(arr);\n    return 0;\n}"
        },
        "structs": {
            "starter_code": "#include <stdio.h>\n\ntypedef struct {\n    // Define struct members\n} Player;\n\nint main() {\n    Player p;\n    // Initialize and use struct\n    return 0;\n}"
        }
    }

    if concept.lower() in concept_customizations:
        custom = concept_customizations[concept.lower()]
        if "starter_code" in custom:
            template["quests"][0]["challenge"]["starter_code"] = custom["starter_code"]

    return template


@mcp.tool
def list_concepts_coverage() -> dict:
    """
    Show which C concepts are covered by levels and identify gaps.

    Returns:
        {
            "covered": [{concept, levels, quest_count}],
            "suggested_gaps": [str]
        }
    """
    data = load_levels()

    if "error" in data:
        return {"error": data["error"]}

    # Expected C concepts for a complete curriculum
    expected_concepts = [
        "variables", "printf", "scanf", "operators",
        "if_else", "loops", "switch",
        "arrays", "strings", "functions",
        "pointers", "pointer_arithmetic", "malloc", "free",
        "structs", "typedef", "enums",
        "file_io", "recursion", "linked_lists"
    ]

    # Track coverage
    covered = {}

    for level in data.get("levels", []):
        level_concept = level.get("concept", "").lower().replace(" ", "_")
        level_id = level.get("id")

        if level_concept:
            if level_concept not in covered:
                covered[level_concept] = {"levels": [], "quest_count": 0}
            covered[level_concept]["levels"].append(level_id)
            covered[level_concept]["quest_count"] += len(level.get("quests", []))

        # Also check quests for concepts
        for quest in level.get("quests", []):
            quest_concept = quest.get("concept", "").lower().replace(" ", "_")
            if quest_concept and quest_concept != level_concept:
                if quest_concept not in covered:
                    covered[quest_concept] = {"levels": [], "quest_count": 0}
                if level_id not in covered[quest_concept]["levels"]:
                    covered[quest_concept]["levels"].append(level_id)
                covered[quest_concept]["quest_count"] += 1

    # Format covered concepts
    covered_list = []
    for concept, data in covered.items():
        covered_list.append({
            "concept": concept,
            "levels": data["levels"],
            "quest_count": data["quest_count"]
        })

    # Find gaps
    covered_concepts = set(covered.keys())
    gaps = [c for c in expected_concepts if c not in covered_concepts]

    return {
        "covered": sorted(covered_list, key=lambda x: x["concept"]),
        "suggested_gaps": gaps
    }


@mcp.tool
def get_quest(level_id: str, quest_id: str) -> dict:
    """
    Get a specific quest from a level.

    Args:
        level_id: Level ID
        quest_id: Quest ID within the level

    Returns:
        Quest object or error
    """
    level = _get_level_internal(level_id)

    if "error" in level:
        return level

    for quest in level.get("quests", []):
        if quest.get("id") == quest_id:
            return quest

    return {"error": f"Quest '{quest_id}' not found in level '{level_id}'"}


if __name__ == "__main__":
    mcp.run()
