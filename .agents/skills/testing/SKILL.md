---
name: testing
description: "Use whenever writing or changing tests in Overhead: unit tests for Rust helpers, fixture tests for response encodings or the session state machine, turning recorded hook traffic into a scrubbed fixture, or reviewing changed insta snapshots. Holds where tests go, the add-only fixture rule, scrubbing for the public repo, and run commands."
---

# Testing (Overhead)

Use with the global `test-design` skill: extend an existing test before
adding one, and prefer fewer, fuller tests.

The MVP tests Rust only. The UI is a renderer of snapshots, so the snapshot
itself is what we pin. There are no frontend unit tests or E2E; check UI
changes by running the app.

## What to test where

| Logic                                                                                            | Test                                                             |
| ------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------- |
| Pure helpers (encoding a decision, parsing a transcript line, diff counts, key matching in Rust) | unit test in the same file, `#[cfg(test)] mod tests`             |
| Session state machine and request lifecycle across many events                                   | fixture replay                                                   |
| Response JSON sent to Claude Code                                                                | fixture replay with an insta snapshot of the response            |
| Panel, focus, hotkey, AppleScript, speech                                                        | manual check in the running app; note what you checked in the PR |

Table-driven unit tests are for the logic where a mistake is costly: response
encodings, state transitions, token check, transcript parsing, tmux/tty
matching.

## Fixtures (`fixtures/`)

A fixture is a recording of real hook traffic from one Claude Code session:
the requests Overhead received, in order, plus any transcript lines the case
needs. The recorder and replay harness come with REK-205; follow its format
once it exists and update this section.

Rules:

- **Add-only.** Never edit, rename or delete an existing fixture: it is what
  Claude Code really sent, and the tests exist to catch us drifting from it.
  If one is wrong, ask the owner. New behaviour gets a new fixture.
- **Scrub before committing.** The repo is public. Replace:
  - the home directory with `/Users/test`
  - project names and prompts with neutral text
  - tokens with `test-token`
  - anything else personal

  Keep field shapes intact.

- Name a fixture after what it shows: `permission-bash-allow.jsonl`,
  `question-multi-select.jsonl`, `answered-in-terminal.jsonl`.
- Replays are deterministic: inject the clock and paths (see `rust-core`);
  never sleep in a test.

## Snapshots (insta)

- Replay a fixture, then `insta::assert_json_snapshot!` the final snapshot
  and every response sent. Snapshots live in `src-tauri/tests/snapshots/`
  (integration) or next to the module (unit).
- A changed `.snap` is a behaviour change. Review the diff with
  `cargo insta review` and accept it only if the new output is what Claude
  Code should receive. Mention accepted snapshot changes in the PR summary.
- Redact unstable fields (timestamps, generated ids) with insta redactions
  rather than removing them from the snapshot.

## Running

```bash
cd src-tauri
cargo test                    # everything
cargo test requests::         # one module
cargo insta test --review     # run and review changed snapshots
```

CI runs the full suite on a macOS runner. Locally, run the tests you touched
when asked to; don't run the whole suite by default.
