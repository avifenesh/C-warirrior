"""
Project Health MCP Server for Code Warrior.
Provides one-call build status for all components.
"""

from fastmcp import FastMCP
import subprocess
import os
from datetime import datetime
from typing import Optional

mcp = FastMCP("Code-Warrior-Health")

PROJECT_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def run_command(cmd: list[str], cwd: str, timeout: int = 30) -> tuple[bool, str]:
    """Run a command and return (success, output)."""
    try:
        result = subprocess.run(
            cmd,
            cwd=cwd,
            capture_output=True,
            text=True,
            timeout=timeout
        )
        return result.returncode == 0, result.stderr or result.stdout
    except subprocess.TimeoutExpired:
        return False, "Command timed out"
    except Exception as e:
        return False, str(e)


def get_last_commit() -> str:
    """Get the last commit info."""
    success, output = run_command(
        ["git", "log", "-1", "--format=%h - %s"],
        PROJECT_ROOT,
        timeout=5
    )
    return output.strip() if success else "unknown"


def count_levels() -> int:
    """Count levels in levels.json."""
    levels_path = os.path.join(PROJECT_ROOT, "src", "assets", "levels.json")
    try:
        import json
        with open(levels_path) as f:
            data = json.load(f)
            # Handle both formats: list or {"levels": [...]}
            if isinstance(data, list):
                return len(data)
            return len(data.get("levels", []))
    except Exception:
        return -1


@mcp.tool
def check_project_health() -> dict:
    """
    One-call build status for all components.

    Returns:
        {
            "rust": "ok" | "error",
            "frontend": "ok" | "error",
            "api": "ok" | "error",
            "level_count": int,
            "last_commit": str,
            "warnings": [str],
            "checked_at": str
        }
    """
    warnings = []

    # Check Rust (shared library)
    rust_ok, rust_out = run_command(
        ["cargo", "check", "--quiet"],
        PROJECT_ROOT
    )
    if not rust_ok and rust_out:
        warnings.append(f"Rust: {rust_out[:200]}")

    # Check Frontend
    frontend_dir = os.path.join(PROJECT_ROOT, "src-ui")
    frontend_ok, frontend_out = run_command(
        ["npm", "run", "check"],
        frontend_dir
    )
    if not frontend_ok and frontend_out:
        warnings.append(f"Frontend: {frontend_out[:200]}")

    # Check API
    api_dir = os.path.join(PROJECT_ROOT, "src-api")
    api_ok, api_out = run_command(
        ["cargo", "check", "--quiet"],
        api_dir
    )
    if not api_ok and api_out:
        warnings.append(f"API: {api_out[:200]}")

    return {
        "rust": "ok" if rust_ok else "error",
        "frontend": "ok" if frontend_ok else "error",
        "api": "ok" if api_ok else "error",
        "level_count": count_levels(),
        "last_commit": get_last_commit(),
        "warnings": warnings,
        "checked_at": datetime.now().isoformat()
    }


@mcp.tool
def quick_build_check(component: str = "all") -> dict:
    """
    Fast check for specific component. Returns first error or ok.

    Args:
        component: "rust" | "frontend" | "api" | "all"

    Returns:
        {
            "status": "ok" | "error",
            "component": str,
            "message": str
        }
    """
    checks = {
        "rust": (["cargo", "check", "--quiet"], PROJECT_ROOT),
        "frontend": (["npm", "run", "check"], os.path.join(PROJECT_ROOT, "src-ui")),
        "api": (["cargo", "check", "--quiet"], os.path.join(PROJECT_ROOT, "src-api"))
    }

    if component == "all":
        for name, (cmd, cwd) in checks.items():
            success, output = run_command(cmd, cwd)
            if not success:
                return {
                    "status": "error",
                    "component": name,
                    "message": output[:300]
                }
        return {
            "status": "ok",
            "component": "all",
            "message": "All components pass"
        }

    if component not in checks:
        return {
            "status": "error",
            "component": component,
            "message": f"Unknown component. Use: {', '.join(checks.keys())}"
        }

    cmd, cwd = checks[component]
    success, output = run_command(cmd, cwd)

    return {
        "status": "ok" if success else "error",
        "component": component,
        "message": "OK" if success else output[:300]
    }


@mcp.tool
def get_git_status() -> dict:
    """
    Get current git status for the project.

    Returns:
        {
            "branch": str,
            "clean": bool,
            "modified_files": [str],
            "untracked_files": [str],
            "last_commit": str
        }
    """
    # Get branch
    success, branch = run_command(
        ["git", "rev-parse", "--abbrev-ref", "HEAD"],
        PROJECT_ROOT,
        timeout=5
    )
    branch = branch.strip() if success else "unknown"

    # Get status
    success, status_out = run_command(
        ["git", "status", "--porcelain"],
        PROJECT_ROOT,
        timeout=5
    )

    modified = []
    untracked = []

    if success and status_out:
        for line in status_out.strip().split('\n'):
            if line.startswith('??'):
                untracked.append(line[3:])
            elif line.strip():
                modified.append(line[3:])

    return {
        "branch": branch,
        "clean": len(modified) == 0 and len(untracked) == 0,
        "modified_files": modified,
        "untracked_files": untracked,
        "last_commit": get_last_commit()
    }


if __name__ == "__main__":
    mcp.run()
