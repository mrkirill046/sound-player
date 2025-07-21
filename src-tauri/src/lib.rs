use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle();
            let window = handle.get_webview_window("main").unwrap();

            #[cfg(not(windows))]
            let _ = window.set_decorations(false);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
