pub mod constants;
pub mod player;
pub mod utils;

pub use constants::{AppState, AUDIO, CURRENT_INDEX, LAST_PATH, PLAYER, PLAYLIST};
pub use player::{
    get_current_audio, next_audio, pause_audio, play_audio, previous_audio, restart_audio,
    resume_audio,
};
pub use utils::{
    build_playlist, frontend_ready, get_audio_cover, is_valid_audio_file, stop_current,
};

use chrono::Local;
use tauri::{Emitter, Manager};
use tauri_plugin_log::{Target, TargetKind};

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
