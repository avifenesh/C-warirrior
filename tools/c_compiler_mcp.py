from fastmcp import FastMCP
import subprocess
import os
import tempfile
import hashlib

# Initialize the MCP Server
mcp = FastMCP("C-Warrior-Runtime")

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

if __name__ == "__main__":
    mcp.run()