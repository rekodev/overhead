---
name: rust-core
description: "Use for any Rust change in Overhead's src-tauri/: adding a Tauri command or event, pushing snapshots to the webview, a new module or file, error types, settings storage, threads and async, adding a crate, or fixing clippy warnings. Holds the feature-module layout, state ownership, tauri-specta bindings, thiserror/anyhow rules and the lint setup."
---

# Rust core (Overhead)

The Rust core owns **all** state. The React webview only renders snapshots
and sends user intents back as commands. If you find yourself deciding
something in TypeScript that changes what Claude Code receives, move it to
Rust.

Use with `rust-best-practices` (idioms, unsafe, errors) and `tauri-v2`
(Tauri APIs). For hooks and native code read `claude-hooks` and
`macos-native`.

## Layout

One module per feature, matching the Linear epics. Native code lives inside
the feature that needs it, not in a shared `platform/` folder.

```
src-tauri/src/
  main.rs        calls overhead_lib::run(); nothing else
  lib.rs         Builder: plugins, setup, managed state, invoke handler
  state/         session state machine, snapshot type
  server/        hook HTTP server, token check
  requests/      pending registry, response encoding
  transcript/    watcher, turn summary
  panel/         nspanel, pill placement, dim overlay
  tray.rs        menu bar icon and its menu
  hotkey/        summon key
  jump/          tmux + Ghostty
  speech/        push-to-talk
  settings/      settings JSON store
  bridge.rs      Tauri commands + snapshot events (tauri-specta)
```

- A module starts as `name.rs`; turn it into `name/mod.rs` + files once it
  has more than one responsibility.
- Keep pure logic (state transitions, encodings, parsing, summaries) in
  functions that take data and return data. I/O (HTTP, files, processes,
  FFI) stays at the edges. Pure logic is what the fixture tests replay.
- Make the clock and the filesystem root injectable where logic depends on
  them, so replays are deterministic.
- `pub(crate)` by default. `unreachable_pub` warns on accidental `pub`.

## State and snapshots

- One owner of session state, in `state/`. Other modules send it events
  (hook arrived, card answered, transcript changed) and it decides.
- After every change, emit one serialisable `Snapshot` to the webview. The
  UI never patches state locally.
- Don't hold a lock across an `.await` or an FFI call. The exact sync
  primitive is chosen in REK-201; follow whatever is there.
- Persist only settings (`settings/`, a JSON file in the app config dir).
  Sessions and requests are memory only.

## Commands and events (`bridge.rs`)

- Types crossing to TS derive `serde::Serialize`/`Deserialize` and
  `specta::Type`. Commands are `#[tauri::command] #[specta::specta]`.
- tauri-specta generates `src/bindings.ts` in debug builds. Never edit it
  by hand; change the Rust type and rebuild. Commit the regenerated file
  with the Rust change so the contract and the UI move together.
- Commands are intents (`allow_request`, `deny_request`, `mark_seen`,
  `jump_to_session`), not setters. Validate ids: the request may already be
  answered in the terminal.
- Add crates (`tauri-specta`, `specta`, `thiserror`, `anyhow`, `insta`) in
  the first change that uses them, not ahead of time.

## Errors

- `thiserror` enums inside a module when callers match on the cause (e.g.
  `RequestError::AlreadySettled`).
- `anyhow` at the edges: setup, process/FFI glue, background tasks, with
  `.context("…")` saying what we were doing.
- Errors returned from commands are a small serialisable enum the UI can
  show. Never send raw `anyhow` strings with paths or payload text.
- No `unwrap()` outside tests; `expect("why this can't fail")` is allowed.
  Background tasks must not panic: log and keep running, since a dead hook
  server blocks every session's prompts until timeout.

## Async and threads

- Use Tauri's runtime (`tauri::async_runtime`), not a second Tokio runtime.
- AppKit calls must run on the main thread
  (`app.run_on_main_thread`). objc2 main-thread-only types enforce this via
  `MainThreadMarker`; don't work around it.

## Lints and checks

`Cargo.toml` carries the BeautifulRust lint set (app profile) and
`clippy.toml` its config. Before handing back Rust code:

```bash
cd src-tauri
cargo fmt
cargo clippy --all-targets -- -D warnings
```

Fix lints rather than silencing them. When one is genuinely wrong, use
`#[expect(clippy::name, reason = "…")]` on the narrowest item.

Comments: code explains itself. Allowed: `// SAFETY:` on unsafe blocks,
`TODO`/`FIXME` with a REK ID, and tool directives. Doc comments only where a
name can't carry the meaning (protocol quirks, units).
