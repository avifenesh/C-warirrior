from fastmcp import FastMCP
import subprocess
import os
import tempfile
import hashlib
from dataclasses import dataclass
from typing import Optional

# Initialize the MCP Server
mcp = FastMCP("C-Warrior-Runtime")


@dataclass
class CompilationResult:
    """Structured compilation result for better context efficiency."""
    success: bool
    stdout: str
    stderr: str
    exit_code: int
    compilation_warnings: list[str]
    timed_out: bool
    error_type: Optional[str]  # "compilation", "runtime", "timeout", None

@mcp.tool
def compile_and_run_c(source_code: str, input_data: str = "") -> str:
    """
    Compiles and executes a snippet of C code using GCC.
    Useful for verifying game logic or checking student code solutions.
    
    Args:
        source_code: The complete C source code (must include main).
        input_data: Optional stdin input to feed to the program.
    """
    
    # Create a temporary directory for compilation
    with tempfile.TemporaryDirectory() as temp_dir:
        # Hash source to create unique filenames
        file_hash = hashlib.md5(source_code.encode()).hexdigest()[:8]
        source_file = os.path.join(temp_dir, f"prog_{file_hash}.c")
        binary_file = os.path.join(temp_dir, f"prog_{file_hash}.exe")

        # Write source code to disk
        with open(source_file, "w") as f:
            f.write(source_code)

        # 1. COMPILE STEP
        # -x c: treat input as C
        # -Wall: enable all warnings
        compile_cmd = ["gcc", "-x", "c", "-Wall", source_file, "-o", binary_file]
        
        try:
            compile_proc = subprocess.run(
                compile_cmd, 
                capture_output=True, 
                text=True, 
                timeout=5
            )
            
            if compile_proc.returncode != 0:
                return f"❌ COMPILATION ERROR:\n{compile_proc.stderr}"

        except subprocess.TimeoutExpired:
            return "❌ COMPILATION TIMEOUT"

        # 2. EXECUTION STEP
        if not os.path.exists(binary_file):
            return "❌ ERROR: Binary not found after compilation."

        try:
            # Run with a strict timeout (2s) to prevent infinite loops
            run_proc = subprocess.run(
                [binary_file],
                input=input_data,
                capture_output=True,
                text=True,
                timeout=2
            )
            
            output = []
            if run_proc.stdout:
                output.append(f"--- STDOUT ---\n{run_proc.stdout}")
            if run_proc.stderr:
                output.append(f"--- STDERR ---\n{run_proc.stderr}")
                
            return "\n".join(output) if output else "✅ Program ran successfully (No Output)."

        except subprocess.TimeoutExpired:
            return "❌ RUNTIME ERROR: Execution timed out (Infinite loop?)"
        except Exception as e:
            return f"❌ SYSTEM ERROR: {str(e)}"

@mcp.tool
def compile_and_run_c_structured(source_code: str, input_data: str = "") -> dict:
    """
    Structured version of compile_and_run_c that returns a dict.
    Better for programmatic use and reduces context pollution.

    Args:
        source_code: The complete C source code (must include main).
        input_data: Optional stdin input to feed to the program.

    Returns:
        {
            "success": bool,
            "stdout": str,
            "stderr": str,
            "exit_code": int,
            "warnings": [str],
            "error_type": "compilation" | "runtime" | "timeout" | null
        }
    """

    with tempfile.TemporaryDirectory() as temp_dir:
        file_hash = hashlib.md5(source_code.encode()).hexdigest()[:8]
        source_file = os.path.join(temp_dir, f"prog_{file_hash}.c")
        binary_file = os.path.join(temp_dir, f"prog_{file_hash}.exe")

        with open(source_file, "w") as f:
            f.write(source_code)

        # Compile
        compile_cmd = ["gcc", "-x", "c", "-Wall", source_file, "-o", binary_file]

        try:
            compile_proc = subprocess.run(
                compile_cmd,
                capture_output=True,
                text=True,
                timeout=5
            )

            # Extract warnings from stderr even if compilation succeeded
            warnings = [line for line in compile_proc.stderr.split('\n')
                       if 'warning:' in line.lower()]

            if compile_proc.returncode != 0:
                return {
                    "success": False,
                    "stdout": "",
                    "stderr": compile_proc.stderr,
                    "exit_code": compile_proc.returncode,
                    "warnings": warnings,
                    "error_type": "compilation"
                }

        except subprocess.TimeoutExpired:
            return {
                "success": False,
                "stdout": "",
                "stderr": "Compilation timed out",
                "exit_code": -1,
                "warnings": [],
                "error_type": "timeout"
            }

        # Execute
        if not os.path.exists(binary_file):
            return {
                "success": False,
                "stdout": "",
                "stderr": "Binary not found after compilation",
                "exit_code": -1,
                "warnings": warnings,
                "error_type": "compilation"
            }

        try:
            run_proc = subprocess.run(
                [binary_file],
                input=input_data,
                capture_output=True,
                text=True,
                timeout=2
            )

            return {
                "success": run_proc.returncode == 0,
                "stdout": run_proc.stdout,
                "stderr": run_proc.stderr,
                "exit_code": run_proc.returncode,
                "warnings": warnings,
                "error_type": "runtime" if run_proc.returncode != 0 else None
            }

        except subprocess.TimeoutExpired:
            return {
                "success": False,
                "stdout": "",
                "stderr": "Execution timed out (possible infinite loop)",
                "exit_code": -1,
                "warnings": warnings,
                "error_type": "timeout"
            }
        except Exception as e:
            return {
                "success": False,
                "stdout": "",
                "stderr": str(e),
                "exit_code": -1,
                "warnings": warnings,
                "error_type": "runtime"
            }


@mcp.tool
def validate_puzzle_suite(
    solution_code: str,
    test_cases: list[dict],
    timeout_per_test: int = 2
) -> dict:
    """
    Validates a C puzzle solution against multiple test cases in a single call.
    More efficient than calling compile_and_run_c multiple times.

    Args:
        solution_code: The complete C solution code (must include main that reads from stdin).
        test_cases: List of {"input": str, "expected": str} test case objects.
        timeout_per_test: Timeout in seconds per test case (default 2).

    Returns:
        {
            "success": bool,           # All tests passed
            "passed": int,             # Number of tests passed
            "failed": int,             # Number of tests failed
            "results": [               # Per-test results
                {"input": str, "expected": str, "actual": str, "passed": bool}
            ],
            "compilation_error": str | None  # If compilation failed
        }
    """

    # Create a temporary directory for compilation
    with tempfile.TemporaryDirectory() as temp_dir:
        file_hash = hashlib.md5(solution_code.encode()).hexdigest()[:8]
        source_file = os.path.join(temp_dir, f"prog_{file_hash}.c")
        binary_file = os.path.join(temp_dir, f"prog_{file_hash}.exe")

        # Write source code
        with open(source_file, "w") as f:
            f.write(solution_code)

        # Compile once
        compile_cmd = ["gcc", "-x", "c", "-Wall", source_file, "-o", binary_file]

        try:
            compile_proc = subprocess.run(
                compile_cmd,
                capture_output=True,
                text=True,
                timeout=5
            )

            if compile_proc.returncode != 0:
                return {
                    "success": False,
                    "passed": 0,
                    "failed": len(test_cases),
                    "results": [],
                    "compilation_error": compile_proc.stderr
                }

        except subprocess.TimeoutExpired:
            return {
                "success": False,
                "passed": 0,
                "failed": len(test_cases),
                "results": [],
                "compilation_error": "Compilation timed out"
            }

        # Run all test cases
        results = []
        passed = 0
        failed = 0

        for test in test_cases:
            input_data = test.get("input", "")
            expected = test.get("expected", "").strip()

            try:
                run_proc = subprocess.run(
                    [binary_file],
                    input=input_data,
                    capture_output=True,
                    text=True,
                    timeout=timeout_per_test
                )

                actual = run_proc.stdout.strip()
                test_passed = actual == expected

                if test_passed:
                    passed += 1
                else:
                    failed += 1

                results.append({
                    "input": input_data,
                    "expected": expected,
                    "actual": actual,
                    "passed": test_passed
                })

            except subprocess.TimeoutExpired:
                failed += 1
                results.append({
                    "input": input_data,
                    "expected": expected,
                    "actual": "TIMEOUT",
                    "passed": False
                })
            except Exception as e:
                failed += 1
                results.append({
                    "input": input_data,
                    "expected": expected,
                    "actual": f"ERROR: {str(e)}",
                    "passed": False
                })

        return {
            "success": passed == len(test_cases),
            "passed": passed,
            "failed": failed,
            "results": results,
            "compilation_error": None
        }


if __name__ == "__main__":
    mcp.run()