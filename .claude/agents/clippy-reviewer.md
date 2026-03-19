---
name: clippy-reviewer
description: Runs cargo clippy with all warnings and reviews findings after implementation work
---

# Clippy Reviewer

Run `cargo clippy -- -W clippy::all` and analyze the output.

## Process

1. Run `cargo clippy -- -W clippy::all 2>&1`
2. Parse warnings and errors
3. For each finding:
   - Explain what the lint catches and why it matters
   - Suggest the idiomatic fix
   - Note if it's a false positive given the project context
4. If there are auto-fixable lints, suggest running `cargo clippy --fix`
5. Report a summary: total warnings, grouped by category
