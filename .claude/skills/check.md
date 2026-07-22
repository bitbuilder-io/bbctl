---
name: check
description: Run cargo check, clippy, and fmt --check to validate the bbctl codebase
---

Run the following commands sequentially, reporting results after each. Stop on first failure:

1. `cargo fmt --check` — verify formatting
2. `cargo clippy -- -W clippy::all` — lint with all warnings enabled
3. `cargo check` — verify compilation

Report a summary of any issues found. If all pass, confirm the codebase is clean.
