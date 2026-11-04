use anyhow::Context;
use tauri::{AppHandle, Manager};
use tauri_nspanel::{CollectionBehavior, PanelLevel, StyleMask, WebviewWindowExt, tauri_panel};

pub(crate) const HUD_LABEL: &str = "hud";

tauri_panel! {
    panel!(HudPanel {
        config: {
            can_become_key_window: true,
            is_floating_panel: true
        }
    })
}

pub(crate) fn init(app: &AppHandle) -> anyhow::Result<()> {
    let window = app
        .get_webview_window(HUD_LABEL)
        .context("the hud window is missing from tauri.conf.json")?;
    let panel = window
        .to_panel::<HudPanel>()
        .context("converting the hud window into a panel")?;

    panel.set_level(PanelLevel::Status.value() + 1);
    panel
        .add_style_mask(StyleMask::empty().nonactivating_panel().into())
        .context("making the hud panel non-activating")?;
    panel.set_collection_behavior(
        CollectionBehavior::new()
            .can_join_all_spaces()
            .full_screen_auxiliary()
            .ignores_cycle()
            .stationary()
            .into(),
    );
    panel.set_hides_on_deactivate(false);
    panel.show();
    Ok(())
}
