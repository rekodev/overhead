use anyhow::{Context, anyhow};
use tauri::{AppHandle, Manager, Wry};
use tauri_nspanel::objc2_app_kit::NSWindowAnimationBehavior;
use tauri_nspanel::{
    CollectionBehavior, ManagerExt, PanelHandle, PanelLevel, StyleMask, WebviewWindowExt,
    tauri_panel,
};

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
    panel
        .as_panel()
        .setAnimationBehavior(NSWindowAnimationBehavior::None);
    panel.show();
    Ok(())
}

pub(crate) fn toggle_focus(app: &AppHandle) -> anyhow::Result<()> {
    on_main_thread(app, |panel| {
        if panel.as_panel().isKeyWindow() {
            release(panel);
        } else {
            take_keys(panel);
        }
    })
}

pub(crate) fn release_focus(app: &AppHandle) -> anyhow::Result<()> {
    on_main_thread(app, release)
}

fn take_keys(panel: &PanelHandle<Wry>) {
    panel.show_and_make_key();
    let webview = panel.content_view().subviews().firstObject();
    if let Some(webview) = webview {
        panel.make_first_responder(Some(&webview));
    }
}

fn release(panel: &PanelHandle<Wry>) {
    panel.make_first_responder(None);
    panel.hide();
    panel.show();
}

fn on_main_thread(
    app: &AppHandle,
    action: impl FnOnce(&PanelHandle<Wry>) + Send + 'static,
) -> anyhow::Result<()> {
    let panel = app
        .get_webview_panel(HUD_LABEL)
        .map_err(|_| anyhow!("the hud panel was never created"))?;
    app.run_on_main_thread(move || action(&panel))
        .context("scheduling a panel call on the main thread")
}
