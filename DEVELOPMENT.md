# Development Guide

This file provides guidance to AI coding assistants and developers when working with code in this repository.

## Project Overview

This is a Rust port of a bash-based JSON list viewer tool. The project displays JSON arrays of objects in a compact, human-readable, colorized terminal format with responsive layout and filtering capabilities.

**Key architectural note**: The entire application is a single-file Rust binary (`rust/json-list/src/main.rs`) with a monolithic design. There are no modules, libraries, or separate components.

## Build and Run Commands

### Build
```bash
cd rust/json-list && cargo build --release
```

The release binary will be at `rust/json-list/target/release/json-list`.

### Run
```bash
cat test.json | ./rust/json-list/target/release/json-list
```

Or use the included test data:
```bash
cat test.json | cargo run --manifest-path rust/json-list/Cargo.toml
```

### Test
```bash
cd rust/json-list && cargo test
```

Run specific test:
```bash
cd rust/json-list && cargo test test_name
```

### Format and Lint
```bash
cd rust/json-list && cargo fmt
cd rust/json-list && cargo clippy
```

## Code Architecture

### Single-File Design
The application (`rust/json-list/src/main.rs`) contains:
- CLI argument parsing using `clap`
- JSON deserialization with `serde_json` and `IndexMap` (preserves field order)
- Terminal output formatting with `colored`
- Responsive layout engine in `process_record()` function

### Layout Algorithm
The `process_record()` function implements a key layout strategy:
1. Calculates effective column widths based on content
2. Wraps columns to new lines when they exceed terminal width
3. Handles "wide" values (longer than terminal width) specially by giving them full-width display
4. Applies color schemes based on column names

### Color Highlighting System
Columns can be highlighted with different colors via CLI flags:
- Primary key (default: `id`) → red
- Highlighted column (default: `name`) → black on white
- Optional: `--yellow`, `--green`, `--magenta`, `--red` for custom columns

### Key Dependencies
- `indexmap` + `serde` → preserves JSON field order during deserialization
- `colored` → terminal color output
- `atty` → detects if output is to a TTY (auto-disables color for pipes)
- `regex` → filtering with `--grep` flag
- `term_size` → auto-detects terminal width

### Release Profile Optimizations
The `Cargo.toml` includes aggressive size optimizations:
- LTO enabled
- Single codegen unit
- Panic abort strategy
- Size optimization (`opt-level = 'z'`)
- Binary stripping

## AI Agent Workflow (Spanish)

**Note**: This project follows a Spanish-language AI collaboration workflow documented in `ai/START_HERE.md`.

When the user indicates a feature is complete and ready to commit:
1. Create a git commit with changes
2. Write a session summary in `ai/journal/YYYY-MM-DD_Session_summary.md`
3. Example: `2025-09-02_Session_summary.md` (with `_Part_2.md`, `_Part_3.md` suffixes if session continues)

## Reference Implementation

The original bash implementation is in `reference-implementation/json-list`. Consult this for behavior reference when making changes.
