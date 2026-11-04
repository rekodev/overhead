use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, include_image};

const TRAY_ID: &str = "overhead";
const PAUSE_ID: &str = "pause";
const SETTINGS_ID: &str = "settings";
const QUIT_ID: &str = "quit";

pub(crate) fn init(app: &AppHandle) -> tauri::Result<()> {
    // TODO(REK-271): wire Pause.
    let pause = MenuItemBuilder::with_id(PAUSE_ID, "Pause")
        .enabled(false)
        .build(app)?;
    // TODO(REK-274): open the Settings window.
    let settings = MenuItemBuilder::with_id(SETTINGS_ID, "Settings…")
        .enabled(false)
        .build(app)?;
    let quit = MenuItemBuilder::with_id(QUIT_ID, "Quit Overhead")
        .accelerator("CmdOrCtrl+Q")
        .build(app)?;
    let menu = MenuBuilder::new(app)
        .items(&[&pause, &settings])
        .separator()
        .item(&quit)
        .build()?;

    TrayIconBuilder::with_id(TRAY_ID)
        .icon(include_image!("icons/tray.png"))
        .icon_as_template(true)
        .tooltip("Overhead")
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| {
            if event.id() == QUIT_ID {
                app.exit(0);
            }
        })
        .build(app)?;
    Ok(())
}
