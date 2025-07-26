pub mod constants;
pub mod player;
pub mod utils;

use std::sync::Mutex;

pub use constants::{AUDIO, CURRENT_INDEX, LAST_PATH, PLAYER, PLAYLIST};
pub use player::{
    get_current_audio, next_audio, pause_audio, play_audio, previous_audio, restart_audio,
    resume_audio,
};
pub use utils::{
    build_playlist, get_audio_cover, handle_initial_argv, is_valid_audio_file, stop_current,
};

use chrono::Local;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_log::{Target, TargetKind};

struct AppState {
    initial_path: Mutex<Option<String>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            initial_path: Mutex::new(None),
        }
    }
}

#[tauri::command]
fn frontend_ready(app: AppHandle, state: tauri::State<'_, AppState>) {
    log::debug!("Invoked frontend_ready function");

    if let Some(path) = state.initial_path.lock().unwrap().take() {
        if let Some(win) = app.get_webview_window("main") {
            let _ = win.emit("open-file", path);

            log::info!("Emit open-file event successfully");
        } else {
            log::error!("Error while emit open-file event");
        }
    } else {
        log::debug!("No path provided in start");
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d_%H-%M-%S").to_string();

    let args: Vec<String> = std::env::args().collect();
    let initial_path = args.get(1).cloned();

    let file_name = format!("{timestamp}.log");
    let state = AppState::new();

    if let Some(path) = initial_path {
        *state.initial_path.lock().unwrap() = Some(path);
    }

    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_fs::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir {
                        file_name: Some(file_name),
                    }),
                ])
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            if let Some(path) = argv.get(1) {
                if let Some(win) = app.get_webview_window("main") {
                    let _ = win.emit("open-file", path.clone());
                }
            }
        }))
        .setup(|app| {
            let handle = app.handle();
            let window = handle.get_webview_window("main").unwrap();

            #[cfg(not(windows))]
            let _ = window.set_decorations(false);

            let handle_clone = handle.clone();

            tauri::async_runtime::spawn(async move {
                std::thread::sleep(std::time::Duration::from_millis(500));

                handle_initial_argv(&handle_clone);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            frontend_ready,
            play_audio,
            pause_audio,
            resume_audio,
            restart_audio,
            next_audio,
            previous_audio,
            get_audio_cover,
            get_current_audio,
            is_valid_audio_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
