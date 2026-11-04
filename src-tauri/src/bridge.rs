use tauri::{AppHandle, Wry};
use tauri_specta::{Builder, collect_commands};

use crate::panel;

#[tauri::command]
#[specta::specta]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Tauri commands receive their arguments by value"
)]
fn release_focus(app: AppHandle) {
    if let Err(err) = panel::release_focus(&app) {
        eprintln!("releasing focus failed: {err:#}");
    }
}

pub(crate) fn builder() -> Builder<Wry> {
    let builder = Builder::<Wry>::new().commands(collect_commands![release_focus]);
    #[cfg(debug_assertions)]
    builder
        .export(
            specta_typescript::Typescript::default(),
            concat!(env!("CARGO_MANIFEST_DIR"), "/../src/bindings.ts"),
        )
        .expect("exporting the TypeScript bindings in debug builds");
    builder
}
