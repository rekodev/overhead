---
name: macos-native
description: "Use before writing any native macOS code in Overhead: the floating HUD panel (tauri-nspanel, fullscreen apps, Spaces, staying on top), focus and Esc handoff, tray and Dock icon, the summon hotkey, jump-back to tmux panes and Ghostty tabs via AppleScript, the dim overlay window, Focus/DND, screen-sharing and lock detection, push-to-talk with Apple Speech, and any objc2, unsafe or FFI code. Lists the known traps and permission rules."
---

# macOS native (Overhead)

Overhead is a background HUD that floats over everything and must never
steal focus. Most bugs in competing apps (Vibe Island, Agent HUD) are in this
layer, so the traps below come from their issue trackers and our own spikes.
When you learn a new one, add it here.

Read `rust-core` for module layout and `rust-best-practices` for unsafe/FFI
rules. Every `unsafe` block gets one operation and a `// SAFETY:` comment
(enforced by clippy).

## The panel (`panel/`)

- The HUD window is an NSPanel made with `tauri-nspanel` v2.1 (plain Tao
  can't make one, [tao 414](https://github.com/tauri-apps/tao/issues/414)).
  Non-activating, floating, can become key.
- **Never call Tauri's `set_focusable` on the panel.** The class swap makes
  Tao panic with "ivar not found".
- Level `NSStatusWindowLevel + 1`. Collection behaviour
  `CanJoinAllSpaces | FullScreenAuxiliary | IgnoresCycle | Stationary`, so it
  shows over fullscreen apps, on every Space, and stays put in Mission Control
  without breaking previews
  ([Agent HUD Swift 134](https://github.com/jazzenchen/agent-hud-native-open/issues/134)).
- Transparent, borderless, no shadow, sized to its content.
- Showing the panel must not change `NSWorkspace.frontmostApplication`.
  Check that whenever you touch show/hide code.
- Fallback if tauri-nspanel blocks us: Agent HUD's Tao fork
  (`jazzenchen/tao`, branch `agent-hud/nonactivating-panel`). Reference for
  level and behaviour: Agent HUD `src/crates/host/hud/macos.rs`.

## Focus

- New requests **never** take the keyboard. The panel becomes key only on
  summon, without activating the app.
- Esc (outside a text field) releases focus to the app that was frontmost
  before summon, and it must get its key window back. Remember that app at
  summon time.
- Activation policy is Accessory: no Dock icon, no app-switcher entry. The
  tray icon is a template image so it fits light and dark menu bars.

## Placement

- Top right of the **active display** (the one with the key window), just
  below the menu bar, never covering menu bar items
  ([VI 180](https://github.com/vibeislandapp/vibe-island/issues/180),
  [VI 132](https://github.com/vibeislandapp/vibe-island/issues/132)).
  Re-place on display or screen-parameter changes.

## Summon hotkey (`hotkey/`)

- The only global shortcut. Default ⌥Space, configurable.
- Carbon `RegisterEventHotKey`: no Accessibility permission. Check first
  whether Tauri's `global-shortcut` plugin matches modifiers exactly; if it
  does, prefer it.
- Exact modifiers only. ⇧⌥Space is the input-source switch and must still
  work ([VI 78](https://github.com/vibeislandapp/vibe-island/issues/78)).
- If registration fails (taken), keep the old key and show an error.
- In-card keys are handled in the webview by character (`event.key`), not
  keycode ([VI 127](https://github.com/vibeislandapp/vibe-island/issues/127)).
  Never synthesize keystrokes into other apps.

## Jump-back (`jump/`)

1. **tmux**, from the `X-Tmux-Pane` header (e.g. `%12`): resolve with
   `tmux display-message -p -t <pane> '#{session_name} #{window_index} #{pane_id}'`,
   find a client with `tmux list-clients -F '#{client_tty} #{client_session}'`,
   then `switch-client`, `select-window`, `select-pane`.
2. **Ghostty 1.4+** via AppleScript: find the terminal whose `tty` matches the
   tmux client tty (or the session's own tty without tmux) and `focus` it.
3. **Anything else:** bring the app forward, mapped from `TERM_PROGRAM` to a
   bundle id (`iTerm.app` → `com.googlecode.iterm2`, `Apple_Terminal` →
   `com.apple.Terminal`, …).

Run tmux with an argument list (`std::process::Command::args`), never a
shell string. Pane ids, ttys and paths come from hook payloads and are
untrusted. Escape anything put into AppleScript.

The first Ghostty jump triggers the Automation permission prompt. If it's
denied, fall back to step 3 and say so once; don't re-prompt in a loop.

## Dim overlay and quiet scenes

- The 3-minute dim is a separate click-through, borderless, full-screen
  window on the active display, ~30% black, just below the panel. Any key or
  click undims it; it must never swallow that input.
- Quiet scenes skip popups, sound, pulse and dim:
  - Focus/DND
  - screen sharing or recording
  - locked screen (`com.apple.screenIsLocked` / `screenIsUnlocked`)
  - fullscreen video
- Detection is best-effort. When a signal is unavailable, assume not quiet
  and log it once, so a missing API never hides a request that needs you.

## Speech (`speech/`)

- `objc2-speech`: `SFSpeechRecognizer` + `SFSpeechAudioBufferRecognitionRequest`
  with `requiresOnDeviceRecognition = true`. If on-device isn't supported for
  the language, show that message; never fall back to network recognition.
- Language: first supported of macOS's preferred languages (LT/EN).
- Microphone and speech permissions are asked on first dictation only.

## Permissions summary

| Permission           | When asked      | Needed for          |
| -------------------- | --------------- | ------------------- |
| Automation (Ghostty) | first jump      | AppleScript `focus` |
| Microphone + Speech  | first dictation | push-to-talk        |
| Accessibility        | never           | avoided on purpose  |

Add the matching usage-description keys to the app's Info.plist in the same
change that first triggers a prompt.
