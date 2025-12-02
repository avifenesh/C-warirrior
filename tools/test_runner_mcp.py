"""
Test Runner MCP Server for Code Warrior.
Integrated testing with smart test selection.
"""

from fastmcp import FastMCP
import subprocess
import os
import json
from datetime import datetime
from typing import Optional

mcp = FastMCP("Code-Warrior-Tests")

PROJECT_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def run_command(cmd: list[str], cwd: str, timeout: int = 120) -> tuple[bool, str, str]:
    """Run a command and return (success, stdout, stderr)."""
    try:
        result = subprocess.run(
            cmd,
            cwd=cwd,
            capture_output=True,
            text=True,
            timeout=timeout
        )
        return result.returncode == 0, result.stdout, result.stderr
    except subprocess.TimeoutExpired:
        return False, "", "Command timed out"
    except Exception as e:
        return False, "", str(e)


@mcp.tool
def run_tests(scope: str = "all") -> dict:
    """
    Run tests for specified scope.

    Args:
        scope: "all" | "rust" | "frontend" | "api" | "e2e"

    Returns:
        {
            "scope": str,
            "success": bool,
            "summary": str,
            "failures": [str],
            "duration_ms": int
        }
    """
    start_time = datetime.now()
    failures = []

    test_configs = {
        "rust": {
            "cmd": ["cargo", "test"],
            "cwd": PROJECT_ROOT,
            "name": "Rust tests"
        },
        "frontend": {
            "cmd": ["npm", "run", "test"],
            "cwd": os.path.join(PROJECT_ROOT, "src-ui"),
            "name": "Frontend tests"
        },
        "api": {
            "cmd": ["cargo", "test"],
            "cwd": os.path.join(PROJECT_ROOT, "src-api"),
            "name": "API tests"
        },
        "e2e": {
            "cmd": ["./tools/test-local-e2e.sh"],
            "cwd": PROJECT_ROOT,
            "name": "E2E tests"
        }
    }

    if scope not in ["all"] + list(test_configs.keys()):
        return {
            "scope": scope,
            "success": False,
            "summary": f"Unknown scope. Use: all, {', '.join(test_configs.keys())}",
            "failures": [],
            "duration_ms": 0
        }

    scopes_to_run = list(test_configs.keys()) if scope == "all" else [scope]
    all_passed = True

    for s in scopes_to_run:
        config = test_configs[s]

        # Check if test command/dir exists
        if not os.path.exists(config["cwd"]):
            failures.append(f"{config['name']}: directory not found")
            all_passed = False
            continue

        success, stdout, stderr = run_command(
            config["cmd"],
            config["cwd"],
            timeout=300  # 5 minutes for tests
        )

        if not success:
            all_passed = False
            # Extract failure summary
            output = stderr or stdout
            # Get last 500 chars as failure summary
            failure_summary = output[-500:] if len(output) > 500 else output
            failures.append(f"{config['name']}: {failure_summary}")

    duration_ms = int((datetime.now() - start_time).total_seconds() * 1000)

    return {
        "scope": scope,
        "success": all_passed,
        "summary": "All tests passed" if all_passed else f"{len(failures)} test suite(s) failed",
        "failures": failures,
        "duration_ms": duration_ms
    }


@mcp.tool
def check_for_regressions(changed_files: list[str]) -> dict:
    """
    Smart test selection based on changed files.

    Args:
        changed_files: List of files that were modified

    Returns:
        {
            "suggested_tests": [str],
            "reason": str,
            "risk_level": "low" | "medium" | "high"
        }
    """
    suggested = set()
    reasons = []
    risk_level = "low"

    for f in changed_files:
        # Rust core logic
        if f.startswith("src/") and f.endswith(".rs"):
            suggested.add("rust")
            suggested.add("api")
            reasons.append("Core Rust logic changed")
            risk_level = "high"

        # API backend
        elif f.startswith("src-api/") and f.endswith(".rs"):
            suggested.add("api")
            reasons.append("API backend changed")
            if risk_level == "low":
                risk_level = "medium"

        # Frontend
        elif f.startswith("src-ui/"):
            suggested.add("frontend")
            if f.endswith(".svelte"):
                reasons.append("Svelte component changed")
            else:
                reasons.append("Frontend code changed")

        # Level data
        elif "levels.json" in f:
            suggested.add("e2e")
            reasons.append("Level data changed")
            risk_level = "high"

        # Test files themselves
        elif "test" in f.lower():
            reasons.append("Test file changed")

    # If high risk, suggest full E2E
    if risk_level == "high":
        suggested.add("e2e")

    return {
        "suggested_tests": list(suggested) if suggested else ["rust"],
        "reason": "; ".join(set(reasons)) if reasons else "No specific changes detected",
        "risk_level": risk_level
    }


@mcp.tool
def run_quick_check() -> dict:
    """
    Run fast checks (compile only, no tests).

    Returns:
        {
            "success": bool,
            "checks": [{component, status, message}]
        }
    """
    checks = []

    # Rust check
    success, stdout, stderr = run_command(
        ["cargo", "check", "--quiet"],
        PROJECT_ROOT,
        timeout=60
    )
    checks.append({
        "component": "rust",
        "status": "ok" if success else "error",
        "message": "" if success else (stderr or stdout)[:200]
    })

    # Frontend check
    success, stdout, stderr = run_command(
        ["npm", "run", "check"],
        os.path.join(PROJECT_ROOT, "src-ui"),
        timeout=60
    )
    checks.append({
        "component": "frontend",
        "status": "ok" if success else "error",
        "message": "" if success else (stderr or stdout)[:200]
    })

    # API check
    success, stdout, stderr = run_command(
        ["cargo", "check", "--quiet"],
        os.path.join(PROJECT_ROOT, "src-api"),
        timeout=60
    )
    checks.append({
        "component": "api",
        "status": "ok" if success else "error",
        "message": "" if success else (stderr or stdout)[:200]
    })

    all_ok = all(c["status"] == "ok" for c in checks)

    return {
        "success": all_ok,
        "checks": checks
    }


@mcp.tool
def validate_c_solution(code: str, test_cases: list[dict]) -> dict:
    """
    Validate a C code solution against test cases.
    Wrapper around c_compiler MCP for convenience.

    Args:
        code: Complete C source code
        test_cases: List of {"input": str, "expected": str}

    Returns:
        {
            "success": bool,
            "passed": int,
            "failed": int,
            "results": [{input, expected, actual, passed}]
        }
    """
    import tempfile
    import hashlib

    with tempfile.TemporaryDirectory() as temp_dir:
        file_hash = hashlib.md5(code.encode()).hexdigest()[:8]
        source_file = os.path.join(temp_dir, f"prog_{file_hash}.c")
        binary_file = os.path.join(temp_dir, f"prog_{file_hash}.exe")

        # Write source
        with open(source_file, 'w') as f:
            f.write(code)

        # Compile
        success, stdout, stderr = run_command(
            ["gcc", "-x", "c", "-Wall", source_file, "-o", binary_file],
            temp_dir,
            timeout=5
        )

        if not success:
            return {
                "success": False,
                "passed": 0,
                "failed": len(test_cases),
                "results": [],
                "compilation_error": stderr
            }

        # Run tests
        results = []
        passed = 0

        for test in test_cases:
            input_data = test.get("input", "")
            expected = test.get("expected", "").strip()

            try:
                proc = subprocess.run(
                    [binary_file],
                    input=input_data,
                    capture_output=True,
                    text=True,
                    timeout=2
                )

                actual = proc.stdout.strip()
                test_passed = actual == expected

                if test_passed:
                    passed += 1

                results.append({
                    "input": input_data,
                    "expected": expected,
                    "actual": actual,
                    "passed": test_passed
                })

            except subprocess.TimeoutExpired:
                results.append({
                    "input": input_data,
                    "expected": expected,
                    "actual": "TIMEOUT",
                    "passed": False
                })
            except Exception as e:
                results.append({
                    "input": input_data,
                    "expected": expected,
                    "actual": f"ERROR: {str(e)}",
                    "passed": False
                })

        return {
            "success": passed == len(test_cases),
            "passed": passed,
            "failed": len(test_cases) - passed,
            "results": results
        }


if __name__ == "__main__":
    mcp.run()
