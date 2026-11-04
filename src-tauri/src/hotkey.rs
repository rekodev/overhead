use anyhow::Context;
use tauri::plugin::TauriPlugin;
use tauri::{AppHandle, Wry};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

use crate::panel;

pub(crate) fn plugin() -> TauriPlugin<Wry> {
    tauri_plugin_global_shortcut::Builder::new()
        .with_handler(|app, _shortcut, event| {
            if event.state() == ShortcutState::Pressed
                && let Err(err) = panel::toggle_focus(app)
            {
                eprintln!("summon failed: {err:#}");
            }
        })
        .build()
}

pub(crate) fn init(app: &AppHandle) -> anyhow::Result<()> {
    // TODO(REK-259): replace with the configurable summon key.
    let summon = Shortcut::new(Some(Modifiers::ALT), Code::Space);
    app.global_shortcut()
        .register(summon)
        .context("registering the ⌥Space summon key")
}
