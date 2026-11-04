mod bridge;
mod hotkey;
mod panel;
mod tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let bridge = bridge::builder();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_nspanel::init())
        .plugin(hotkey::plugin())
        .invoke_handler(bridge.invoke_handler())
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            tray::init(app.handle())?;
            panel::init(app.handle())?;
            if let Err(err) = hotkey::init(app.handle()) {
                eprintln!("{err:#}");
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
