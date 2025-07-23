mod player;
mod utils;

use chrono::Local;
use player::{pause_audio, play_audio, restart_audio, resume_audio};
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};
use utils::get_audio_cover;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d_%H-%M-%S").to_string();

    let file_name = format!("{timestamp}.log");

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Webview),
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir {
                        file_name: Some(file_name),
                    }),
                ])
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let handle = app.handle();
            let window = handle.get_webview_window("main").unwrap();

            #[cfg(not(windows))]
            let _ = window.set_decorations(false);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            play_audio,
            pause_audio,
            resume_audio,
            restart_audio,
            get_audio_cover
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
