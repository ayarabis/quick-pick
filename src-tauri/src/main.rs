#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};

mod spotlight;

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            spotlight::init_spotlight_window,
            spotlight::hide_window
        ])
        .system_tray(system_tray)
        .on_system_tray_event(|_app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .setup(move |app| {
            // Set the app's activation poicy to Accessory does the following behaviours:
            // - Makes the windows of this app appear above full-screen windows of other apps.
            // - Prevents the app's icon from showing on the dock.
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
