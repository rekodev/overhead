---
name: claude-hooks
description: "Use before changing anything in Overhead that affects what Claude Code receives or how sessions and cards behave: plugin/ (hooks.json, register.ts), the hook server, PermissionRequest cards and their actions (allow, deny, feedback, always allow, auto mode, question answers, plan decisions), clearing cards answered in the terminal, releasing held requests on quit, session states, subagents, transcripts and fixtures. Holds the protocol: hook events, payloads, token, held responses and every response encoding."
---

# Claude Code integration (Overhead)

Overhead's whole value is answering Claude Code correctly. A wrong response
encoding silently allows or denies a real tool call in someone's repo, so
treat these rules like a protocol, not style. When a change seems to need
bending one, stop and explain the tradeoff instead of working around it.

Sources of truth, in order: recorded fixtures in `fixtures/`, the Linear
spec ("Overhead — Product spec", sections 4 and 8), then the Claude Code
docs ([hooks](https://code.claude.com/docs/en/hooks),
[plugins reference](https://code.claude.com/docs/en/plugins-reference)).
Claude Code changes often: when a doc and a fixture disagree, the fixture
(what Claude Code actually sent) wins, and the spec gets updated.

## Promises we make users

These are in the README and the spec. Code must keep them true.

- **Never touch their setup.** Overhead installs only through the
  `claude plugin` CLI. It never writes `settings.json`, hooks, statusline,
  output style or terminal titles. The CLI itself writes
  `extraKnownMarketplaces` and `enabledPlugins`.
- **Uninstall leaves nothing.** Anything we add must have a removal path.
- **Nothing leaves the Mac.** The server binds `127.0.0.1` only.
- **Never block Claude.** If Overhead quits, crashes or doesn't know the
  answer, Claude Code's own terminal prompt must take over.

## The plugin (`plugin/`)

- `plugin/hooks/hooks.json`: `http` hooks to
  `http://127.0.0.1:<port>/hook/<event>` for PermissionRequest, Notification,
  Stop, SubagentStart, SubagentStop, SessionStart, SessionEnd,
  UserPromptSubmit and PostToolUse.
- PermissionRequest has `timeout: 86400`, so we can hold it while the user is
  away. Every other hook should answer immediately (empty 200).
- Headers: `X-Overhead-Token: ${CLAUDE_PLUGIN_OPTION_TOKEN}` and
  `X-Tmux-Pane: $TMUX_PANE`, both listed in `allowedEnvVars`. Without
  `allowedEnvVars` the variables are not expanded.
- `plugin/hooks/register.ts`: the hooks module for typed replies. It reads
  queued prompts from `${CLAUDE_PLUGIN_DATA}/prompts/<sessionId>/` and calls
  `$.prompt.submit` when the session is idle. Its exact behaviour is
  verified in REK-254; record what you learn here.
- Install: `claude plugin marketplace add <bundle>/plugin-marketplace`, then
  `claude plugin install overhead@overhead --config token=<random>`. The
  token is generated per install and stored only in the app's settings.
- `claude plugin validate` runs in CI (REK-199). Run it after any change
  under `plugin/`.

## The hook server (`server/`)

- Reject any request without the right `X-Overhead-Token` with 401 and no
  body detail. Compare in constant time.
- Every payload carries `session_id`, `transcript_path`, `cwd` and
  `hook_event_name`. Parse into typed structs, but tolerate unknown fields:
  Claude Code adds fields often, and a strict parse would drop real requests.
- Payload text (prompts, commands, file paths, Claude's messages) is
  untrusted: render it as text, never as HTML, and never put it in a shell
  or AppleScript string without escaping.

## Sessions (`state/`)

States: `working`, `needs-you`, `done-unseen`, `done-seen`, `ended`.

| Trigger                                                       | Result                                    |
| ------------------------------------------------------------- | ----------------------------------------- |
| `UserPromptSubmit`                                            | working                                   |
| `PermissionRequest`                                           | needs-you, card added to the queue        |
| Card answered (HUD, or terminal via transcript `tool_result`) | back to working                           |
| `Stop`                                                        | done-unseen                               |
| X, jump or reply                                              | done-seen                                 |
| 30 min inactive after seen                                    | removed from the list                     |
| `SessionEnd`, or the process is gone                          | ended; its cards cleared, session removed |

List interactive sessions only. Skip `claude -p`,
`CLAUDE_CODE_SESSION_ATTENDED=0`, background summarizers and folders matched
by silence rules (REK-204 records how each is detected).

Subagents (SubagentStart/Stop) nest under their parent session and never
create their own row.

## Holding a request (`requests/`)

1. A PermissionRequest arrives. Keep its HTTP response open, up to 24h.
2. The card shows. Claude Code shows its own terminal prompt **at the same
   time**; whichever is answered first wins.
3. HUD answer → send it as the HTTP response, then drop the card.
4. Terminal answer → the transcript shows a `tool_result` for that
   `tool_use_id`; send an empty 200 and drop the card.
5. App quit → every held request gets an empty 200 first.

A held request must always end in exactly one response. Any new code path
(errors, session end, duplicate request) has to settle it.

## Response encodings

All decisions are wrapped as:

```json
{ "hookSpecificOutput": { "hookEventName": "PermissionRequest", "decision": { … } } }
```

| Action                       | `decision`                                                                                                                                   |
| ---------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| Allow once                   | `{ "behavior": "allow" }`                                                                                                                    |
| Deny                         | `{ "behavior": "deny" }`                                                                                                                     |
| Deny with feedback           | `{ "behavior": "deny", "message": "<text>" }`                                                                                                |
| Always allow                 | `allow` + `updatedPermissions: [<Claude's addRules suggestion, unchanged>]`                                                                  |
| Allow + auto mode            | `allow` + `updatedPermissions: [{ "type": "setMode", "mode": "auto", "destination": "session" }]`                                            |
| Question answers             | `allow` + `updatedInput`: the original input plus `answers: { "<question text>": "label, label, own text" }`; a skipped question gets no key |
| Plan approve                 | `allow` + `updatedInput` (the plan input)                                                                                                    |
| Plan approve + accept edits  | as above + `setMode` `acceptEdits`                                                                                                           |
| Keep planning                | `{ "behavior": "deny", "message": "<feedback>" }`                                                                                            |
| Release / answered elsewhere | empty 200                                                                                                                                    |

Rules:

- Always allow appears only when the request has a suggestion. Echo it back
  unchanged; never build a rule ourselves.
- Never send Bypass mode. Auto mode is the most we offer.
- The exact `mode` value for auto is confirmed in REK-228; update this table
  with what the fixture shows.
- Every encoding has a fixture test (REK-235). Adding or changing one means
  adding a fixture and an insta snapshot (see the `testing` skill).

## Transcripts (`transcript/`)

- Path: `~/.claude/projects/<cwd-slug>/<sessionId>.jsonl`, subagents under
  `subagents/`. Use `transcript_path` from the payload when present instead
  of rebuilding it. Worktree paths are encoded differently
  ([VI 150](https://github.com/vibeislandapp/vibe-island/issues/150)).
- Read only what we need: `tool_result` ids (to clear cards),
  `stop_reason`, Edit/Write tool calls (turn summary +/− counts), the model,
  and the last assistant text.
- Lines are appended while we read. Handle a partial last line and files
  that disappear. Never load a whole long transcript on every change; keep
  an offset.
