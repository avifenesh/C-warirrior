# Large Codebase Analysis with Gemini CLI

**When analyzing large code chunks or understanding the whole codebase**, use Gemini's massive context window:

```bash
gemini -m gemini-3-pro-preview "<prompt>"
```

**ALWAYS use `gemini-3-pro-preview`** - this is the required model.

## When to Use Gemini CLI

- Analyzing files that exceed your context window
- Understanding cross-cutting concerns across many files
- Reviewing large PRs or diffs
- Getting a holistic view of system architecture
- Tracing data flow across the entire codebase

## Best Practices for Gemini Prompts

1. **Structure your prompt clearly** - Use sections/headers:
   ```bash
   gemini -m gemini-3-pro-preview "## Task
   Analyze the data flow in this codebase.

   ## Context
   This is a Rust (Axum) game with Svelte frontend.

   ## Output Format
   Provide a numbered list of the flow steps."
   ```

2. **Place large context first** - Put code/files at the beginning, then ask your question:
   ```bash
   gemini -m gemini-3-pro-preview "$(cat src/*.rs)

   Based on the code above, identify all HTTP endpoints and their purposes."
   ```

3. **Be specific about output format** - Request structured output:
   - "Return as JSON with keys: function, purpose, dependencies"
   - "Provide a bullet list of findings"
   - "Create a markdown table of X vs Y"

4. **Break complex analysis into steps** - Ask Gemini to plan first:
   ```bash
   gemini -m gemini-3-pro-preview "First, list all the modules. Then, for each module, describe its responsibility. Finally, draw the dependency graph."
   ```

5. **Use self-critique for accuracy** - Ask it to verify:
   ```bash
   gemini -m gemini-3-pro-preview "Analyze this code for bugs. After listing potential issues, review each one and rate your confidence (high/medium/low)."
   ```

## Example Use Cases

```bash
# Understand entire codebase structure
gemini -m gemini-3-pro-preview "$(find src -name '*.rs' -exec cat {} \;)

Analyze this Rust codebase and provide:
1. High-level architecture overview
2. Module dependency graph
3. Key data structures and their relationships"

# Trace a specific feature
gemini -m gemini-3-pro-preview "$(cat src/**/*.rs src-ui/src/**/*.ts)

Trace how player movement works from frontend input to backend state update. List every file and function involved."
```

