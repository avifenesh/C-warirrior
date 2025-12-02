"""
Memory MCP Server for Code Warrior.
Cross-session memory for decisions, patterns, and gotchas.
"""

from fastmcp import FastMCP
import json
import os
from datetime import datetime
from typing import Optional

mcp = FastMCP("Code-Warrior-Memory")

PROJECT_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
MEMORY_FILE = os.path.join(PROJECT_ROOT, ".claude", "memory.json")

# Valid categories
CATEGORIES = ["decision", "pattern", "gotcha", "todo", "general"]


def ensure_memory_file():
    """Ensure memory file exists."""
    os.makedirs(os.path.dirname(MEMORY_FILE), exist_ok=True)
    if not os.path.exists(MEMORY_FILE):
        with open(MEMORY_FILE, 'w') as f:
            json.dump({
                "version": "1.0",
                "memories": {}
            }, f, indent=2)


def load_memory() -> dict:
    """Load memory data."""
    ensure_memory_file()
    try:
        with open(MEMORY_FILE, 'r') as f:
            return json.load(f)
    except Exception:
        return {"version": "1.0", "memories": {}}


def save_memory(data: dict):
    """Save memory data."""
    ensure_memory_file()
    with open(MEMORY_FILE, 'w') as f:
        json.dump(data, f, indent=2)


@mcp.tool
def remember(key: str, value: str, category: str = "general") -> str:
    """
    Store a fact/decision for later retrieval.

    Args:
        key: Unique identifier for this memory
        value: The information to remember
        category: "decision" | "pattern" | "gotcha" | "todo" | "general"

    Returns:
        Confirmation message
    """
    if category not in CATEGORIES:
        return f"Invalid category. Use: {', '.join(CATEGORIES)}"

    data = load_memory()

    # Create category if doesn't exist
    if category not in data["memories"]:
        data["memories"][category] = {}

    data["memories"][category][key] = {
        "value": value,
        "created_at": datetime.now().isoformat(),
        "updated_at": datetime.now().isoformat()
    }

    save_memory(data)
    return f"Remembered '{key}' in category '{category}'"


@mcp.tool
def recall(key: str = None, category: str = None) -> dict:
    """
    Retrieve stored memories.

    Args:
        key: Specific key to retrieve (optional)
        category: Filter by category (optional)

    Returns:
        {
            "memories": [{key, value, category, created_at}]
        }
    """
    data = load_memory()
    results = []

    for cat, memories in data.get("memories", {}).items():
        if category and cat != category:
            continue

        for k, v in memories.items():
            if key and k != key:
                continue

            results.append({
                "key": k,
                "value": v.get("value"),
                "category": cat,
                "created_at": v.get("created_at"),
                "updated_at": v.get("updated_at")
            })

    return {"memories": results}


@mcp.tool
def forget(key: str, category: str = None) -> str:
    """
    Remove a memory.

    Args:
        key: Key to remove
        category: Category to remove from (searches all if not specified)

    Returns:
        Confirmation message
    """
    data = load_memory()
    removed = False

    for cat in list(data.get("memories", {}).keys()):
        if category and cat != category:
            continue

        if key in data["memories"].get(cat, {}):
            del data["memories"][cat][key]
            removed = True
            break

    if removed:
        save_memory(data)
        return f"Forgot '{key}'"
    else:
        return f"Memory '{key}' not found"


@mcp.tool
def get_project_decisions() -> dict:
    """
    List all recorded architectural decisions and their rationale.

    Returns:
        {
            "decisions": [{key, value, created_at}]
        }
    """
    data = load_memory()
    decisions = []

    for key, memory in data.get("memories", {}).get("decision", {}).items():
        decisions.append({
            "key": key,
            "value": memory.get("value"),
            "created_at": memory.get("created_at")
        })

    # Sort by creation date (newest first)
    decisions.sort(key=lambda x: x.get("created_at", ""), reverse=True)

    return {"decisions": decisions}


@mcp.tool
def get_known_gotchas() -> dict:
    """
    List known issues/gotchas from previous sessions.

    Returns:
        {
            "gotchas": [{key, value, created_at}]
        }
    """
    data = load_memory()
    gotchas = []

    for key, memory in data.get("memories", {}).get("gotcha", {}).items():
        gotchas.append({
            "key": key,
            "value": memory.get("value"),
            "created_at": memory.get("created_at")
        })

    return {"gotchas": gotchas}


@mcp.tool
def get_patterns() -> dict:
    """
    List recorded code patterns.

    Returns:
        {
            "patterns": [{key, value, created_at}]
        }
    """
    data = load_memory()
    patterns = []

    for key, memory in data.get("memories", {}).get("pattern", {}).items():
        patterns.append({
            "key": key,
            "value": memory.get("value"),
            "created_at": memory.get("created_at")
        })

    return {"patterns": patterns}


@mcp.tool
def search_memory(query: str) -> dict:
    """
    Search across all memories.

    Args:
        query: Search term (case-insensitive)

    Returns:
        {
            "matches": [{key, value, category, relevance}]
        }
    """
    data = load_memory()
    matches = []
    query_lower = query.lower()

    for cat, memories in data.get("memories", {}).items():
        for key, memory in memories.items():
            value = memory.get("value", "")

            # Check if query matches key or value
            key_match = query_lower in key.lower()
            value_match = query_lower in value.lower()

            if key_match or value_match:
                # Simple relevance scoring
                relevance = 0
                if key_match:
                    relevance += 2
                if value_match:
                    relevance += 1

                matches.append({
                    "key": key,
                    "value": value,
                    "category": cat,
                    "relevance": relevance
                })

    # Sort by relevance
    matches.sort(key=lambda x: x["relevance"], reverse=True)

    return {"matches": matches[:20]}


@mcp.tool
def clear_category(category: str) -> str:
    """
    Clear all memories in a category.

    Args:
        category: Category to clear

    Returns:
        Confirmation message
    """
    if category not in CATEGORIES:
        return f"Invalid category. Use: {', '.join(CATEGORIES)}"

    data = load_memory()

    count = len(data.get("memories", {}).get(category, {}))
    data["memories"][category] = {}

    save_memory(data)
    return f"Cleared {count} memories from '{category}'"


@mcp.tool
def get_memory_stats() -> dict:
    """
    Get statistics about stored memories.

    Returns:
        {
            "total": int,
            "by_category": {category: count}
        }
    """
    data = load_memory()

    stats = {"total": 0, "by_category": {}}

    for cat, memories in data.get("memories", {}).items():
        count = len(memories)
        stats["by_category"][cat] = count
        stats["total"] += count

    return stats


if __name__ == "__main__":
    mcp.run()
