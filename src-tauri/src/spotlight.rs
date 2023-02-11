use std::borrow::Borrow;

use cocoa::{
    appkit::{NSMainMenuWindowLevel, NSWindow, NSWindowCollectionBehavior},
    base::id,
    foundation::NSRect,
};
use enigo::Enigo;

use tauri::{
    GlobalShortcutManager, LogicalPosition, LogicalSize, Manager, Monitor, Window, Wry,
};

static mut INIT: bool = false;

#[tauri::command]
pub fn init_spotlight_window(window: Window<Wry>) {
    if window.label() == "main" {
        unsafe {
            if INIT {
                return;
            };
        }
        register_shortcut(&window);
        set_spotlight_window_collection_behaviour(&window);
        window.set_focus().unwrap();
    };
    set_window_above_menubar(&window);
    unsafe {
        INIT = true;
    }
}

fn register_shortcut(window: &Window<Wry>) {
    let window = window.to_owned();
    let mut shortcut_manager = window.app_handle().global_shortcut_manager();
    shortcut_manager
        .register("Control+Shift+Space", move || {
            if window.is_visible().unwrap() {
                window.hide().unwrap();
            } else {
                center_window_at_cursor_monitor(&window);
                window.set_focus().unwrap();
            }
        })
        .unwrap();
}

#[tauri::command]
pub async fn hide_window(window: tauri::Window) {
    let mut size = window.outer_size().unwrap();
    size.height = 92;
    window.set_size(size).unwrap();
    window.hide().unwrap();
}

/// Positions a given window at the center of the monitor with cursor
fn center_window_at_cursor_monitor(window: &Window<Wry>) {
    if let Some(monitor) = get_monitor_with_cursor(window) {
        let screen_size: LogicalSize<u32> = monitor
            .size()
            .borrow()
            .clone()
            .to_logical(monitor.scale_factor());
        let screen_position = monitor.position();

        let handle: id = window.ns_window().unwrap() as _;
        let win_frame: NSRect = unsafe { handle.frame() };

        let x = (screen_position.x as i32 + (screen_size.width as i32 / 2))
            - win_frame.size.width as i32 / 2;
        let y = ((screen_size.height as f32) * 0.3 as f32) + screen_position.y as f32;

        window
            .set_position(LogicalPosition { x, y: y as i32 })
            .unwrap();
        window
            .set_size(LogicalSize {
                width: win_frame.size.width,
                height: 157 as f64,
            })
            .unwrap();
    }
}

/// Set the behaviours that makes the window appear on all worksapces
fn set_spotlight_window_collection_behaviour(window: &Window<Wry>) {
    let handle: id = window.ns_window().unwrap() as _;
    unsafe {
        handle.setCollectionBehavior_(
            NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
                | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary
                | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenPrimary
                | NSWindowCollectionBehavior::NSWindowCollectionBehaviorIgnoresCycle,
        );
    };
}

/// Set the window above menubar level
fn set_window_above_menubar(window: &Window<Wry>) {
    let handle: id = window.ns_window().unwrap() as _;
    unsafe { handle.setLevel_((NSMainMenuWindowLevel + 2).into()) };
}

/// Returns the Monitor with cursor
fn get_monitor_with_cursor(window: &Window<Wry>) -> Option<Monitor> {
    let cursor_location: (i32, i32) = Enigo::mouse_location();

    let screens = window.available_monitors().unwrap();
    let index = screens
        .iter()
        .position(|s| {
            let size: LogicalSize<i32> = s.size().to_logical(s.scale_factor());
            let in_x =
                (s.position().x..(s.position().x + size.width as i32)).contains(&cursor_location.0);
            let in_y = (s.position().y..(s.position().y + size.height) as i32)
                .contains(&cursor_location.1);
            return in_x && in_y;
        })
        .unwrap();
    let screen = screens[index].borrow().clone();
    return Some(screen);
}
