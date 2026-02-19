// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

#[derive(Serialize, Deserialize, Default)]
struct WindowState {
    width: Option<f64>,
    height: Option<f64>,
    x: Option<f64>,
    y: Option<f64>,
}

fn state_file_path(app: &tauri::AppHandle) -> PathBuf {
    app.path()
        .app_config_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("window-state.json")
}

fn load_window_state(app: &tauri::AppHandle) -> WindowState {
    let path = state_file_path(app);
    fs::read_to_string(&path)
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default()
}

fn save_window_state(app: &tauri::AppHandle, window: &tauri::Window) {
    let path = state_file_path(app);
    if let Ok(pos) = window.outer_position() {
        if let Ok(size) = window.outer_size() {
            let state = WindowState {
                width: Some(size.width as f64),
                height: Some(size.height as f64),
                x: Some(pos.x as f64),
                y: Some(pos.y as f64),
            };
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::write(&path, serde_json::to_string(&state).unwrap_or_default());
        }
    }
}

fn create_window(app: &tauri::AppHandle, label: &str, state: WindowState) {
    let width = state.width.unwrap_or(800.0);
    let height = state.height.unwrap_or(600.0);

    let mut builder = WebviewWindowBuilder::new(app, label, WebviewUrl::External("https://linear.app/login".parse().unwrap()))
        .title("Linear")
        .inner_size(width, height);

    if let (Some(x), Some(y)) = (state.x, state.y) {
        builder = builder.position(x, y);
    } else {
        builder = builder.center();
    }

    let _ = builder.build();
}

static WINDOW_COUNTER: AtomicU32 = AtomicU32::new(1);

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let state = load_window_state(app.handle());
            create_window(app.handle(), "main", state);

            // Register Ctrl+Shift+N for new windows
            let shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyN);
            app.global_shortcut().on_shortcut(shortcut, move |app, _shortcut, event| {
                if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                    let mut state = WindowState::default();

                    // Offset from the focused window if one exists
                    if let Some(focused) = app.webview_windows().values().find(|w| w.is_focused().unwrap_or(false)) {
                        if let (Ok(pos), Ok(size)) = (focused.outer_position(), focused.outer_size()) {
                            state = WindowState {
                                width: Some(size.width as f64),
                                height: Some(size.height as f64),
                                x: Some(pos.x as f64 + 30.0),
                                y: Some(pos.y as f64 + 30.0),
                            };
                        }
                    }

                    let id = WINDOW_COUNTER.fetch_add(1, Ordering::Relaxed);
                    create_window(app, &format!("window-{}", id), state);
                }
            })?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                save_window_state(window.app_handle(), window);
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
