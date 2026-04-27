# Notes CLI Agent Context

## Project Goal

This project is a Rust learning exercise that builds a simple command-line notes cache.
The intended product direction is:

- store short text notes and links
- organize entries by category
- add entries interactively
- list entries by category

The implementation is intentionally staged so each Rust concept is introduced in a small, understandable step.

## Current Implementation Status

The project currently implements Step 1 and Step 2 of the learning plan.

### Step 1: Command Dispatch

The CLI currently supports these top-level commands:

- `add`
- `list`

Unknown or missing commands show a help message.

`src/main.rs` currently handles:

- reading CLI args with `std::env::args()`
- matching the first user command
- delegating command execution to small functions

### Step 2: Interactive Prompt Layer

The CLI now supports interactive input for `add` and `list`.

Current behavior:

- `notes add`
  - prompts for type: `text` or `link`
  - prompts for category
  - prompts for content
  - prints the captured values back to the user
- `notes list`
  - prompts for category
  - prints the selected category back to the user

Current validation rules:

- item type must be `text` or `link`
- category cannot be empty
- text content cannot be empty
- link URL cannot be empty

Current limitation:

- nothing is persisted yet
- `add` only captures and echoes user input
- `list` only captures and echoes the selected category

## Rust Concepts Introduced So Far

The code currently demonstrates these Rust concepts:

- `Vec<String>` for CLI arguments
- `Option` from `args.get(1)`
- `match` for command dispatch and type branching
- small functions for separation of concerns
- `std::io` for terminal input/output
- `io::Result<()>` for fallible command handlers
- `loop` for re-prompting invalid input
- `String` and `&str` conversion via `as_str()`
- `?` for error propagation

## Current File Layout

- `src/main.rs`
  - command dispatch
  - `add` and `list` handlers
  - prompt helpers
  - help output

There are no additional modules yet.

## Current Runtime Flow

### `add`

1. Parse the first CLI command.
2. Enter `handle_add()`.
3. Prompt for item type.
4. Prompt for category.
5. Prompt for content based on type.
6. Print captured values.

### `list`

1. Parse the first CLI command.
2. Enter `handle_list()`.
3. Prompt for category.
4. Print the selected category.

## Next Planned Step

The next implementation step is to introduce explicit domain types before persistence.

Recommended types:

- `struct Store { entries: Vec<Entry> }`
- `struct Entry { category: String, item: Item }`
- `enum Item { Text { content: String }, Link { url: String } }`

This step should convert the prompt output from loose strings into real Rust data structures.

## Planned Steps After That

- add JSON persistence using `serde` and `serde_json`
- save entries to a project-local `notes.json`
- load and filter entries during `list`
- refactor into small modules after the MVP works

## Constraints And Defaults

- keep the learning path incremental
- avoid introducing advanced crates too early
- use interactive prompts, not flag parsing, for the MVP
- categories are user-entered strings
- category matching will be exact and case-sensitive in the MVP
- supported item types are currently limited to `text` and `link`

## Notes For Future Codex Sessions

- preserve the staged learning approach
- explain each implementation step and the Rust concepts introduced
- avoid jumping ahead to later planned steps unless explicitly requested
- prefer simple, readable code over abstraction-heavy designs
