# Overhead Agent Guide

## Product

Overhead is a calm, keyboard-first HUD for Claude Code on macOS. A small pill at the top right shows every interactive session; summoning it (⌥Space) lets you approve permission requests, answer questions, review plans, reply, and jump back to the exact Ghostty + tmux pane, all without leaving the app you're in. It is open source (MIT) and Claude Code only.

Principles: keyboard-first, calm by default (only things blocked on you interrupt), Claude Code depth over agent breadth, never touches the user's setup (installs through Claude Code's plugin system and never writes settings.json, hooks, statusline, output style or terminal titles), and never steals focus.

MVP non-goals: other agents, quotas/tokens, phone/Watch, SSH, Bypass mode, extra global shortcuts, mascots or sound packs, a Chrome extension, event history, signing, a website. Don't introduce these as side effects. The spec is the Linear document "Overhead — Product spec"; tickets live in the Linear project Overhead (team REK).

## Repository

- `src-tauri/`: Rust core (Tauri 2). Owns all state, the hook server, native macOS code.
- `src/`: React 19 + TypeScript + Tailwind v4 webview (Vite). Renders snapshots from Rust.
- `plugin/`: the Claude Code plugin (`hooks/hooks.json`, `hooks/register.ts`, local marketplace).
- `fixtures/`: recorded, scrubbed hook traffic for replay tests.

## Always

- Rust owns state. The UI renders snapshots and sends commands; TS types come from the generated `src/bindings.ts` (tauri-specta), never hand-copied.
- Read `claude-hooks` before changing anything that decides what Claude Code receives. A held PermissionRequest always ends in exactly one response, and quitting releases every one.
- Read `macos-native` before touching panels, focus, hotkeys, other apps or any `unsafe`/objc2 code.
- Text from hook payloads and transcripts (prompts, commands, paths, Claude's messages) is untrusted: render as text, pass to processes as arguments, escape for AppleScript.
- Fixtures are add-only and scrubbed of personal data before committing; the repo is public.
- Code explains itself; don't add comments. Allowed: `// SAFETY:` on unsafe blocks, `TODO`/`FIXME` with a REK ID, and tool directives (`eslint-disable`, `@ts-expect-error`, `#[expect(..., reason = "…")]`).
- Don't add documentation files (runbooks, `docs/`) to the repo beyond the README. Planning lives in Linear.
- There is no pre-commit hook. Before committing, check what changed:
  - TypeScript: `pnpm lint` and `pnpm typecheck`
  - Formatting: `pnpm format:check`
  - Rust: `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings`, run in `src-tauri/`
  - Plugin: `claude plugin validate` on `plugin/`

  Tests are left to CI unless asked.

## Skills

| Task                                                                                 | Skill                               |
| ------------------------------------------------------------------------------------ | ----------------------------------- |
| Plugin, hook server, requests, response encodings, transcripts, session states       | `claude-hooks`                      |
| Panel, focus, hotkey, jump-back, dim overlay, quiet scenes, speech, any objc2/unsafe | `macos-native`                      |
| Rust module layout, state, commands/events, errors, dependencies                     | `rust-core` + `rust-best-practices` |
| Tauri APIs, config, capabilities, updater                                            | `tauri-v2`                          |
| Anything under `src/`: components, store, keyboard, styling                          | `ui`                                |
| Writing or extending tests, recording fixtures, insta snapshots                      | `testing`                           |
| Branches, commits, PRs, Linear issues and docs                                       | `linear-git`                        |

`rust-best-practices` and `tauri-v2` are installed with `npx skills` (see `skills-lock.json`); update them with `npx skills update`, don't edit them by hand.
