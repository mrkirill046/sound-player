use crate::{CURRENT_INDEX, PLAYER, PLAYLIST};

use base64::engine::general_purpose;
use base64::Engine as _;
use lofty::{read_from_path, PictureType, TaggedFileExt};
use std::path::Path;
use tauri::{AppHandle, Emitter, Manager};

#[tauri::command]
pub fn get_audio_cover(path: String) -> Result<Option<String>, String> {
    log::debug!("Invoked get_audio_cover function with path: {}", path);

    let tagged_file = read_from_path(&path).map_err(|e| e.to_string())?;
    let tag = tagged_file.primary_tag().ok_or("No tag found")?;

    if let Some(picture) = tag
        .pictures()
        .iter()
        .find(|pic| pic.pic_type() == PictureType::CoverFront)
    {
        let mime = picture
            .mime_type()
            .map(|m| m.as_str())
            .unwrap_or("image/jpeg");

        let encoded = general_purpose::STANDARD.encode(picture.data());
        let base64_url = format!("data:{};base64,{}", mime, encoded);

        log::info!("Audio cover founded successfully");

        Ok(Some(base64_url))
    } else {
        log::info!("Audio cover not founded");

        Ok(None)
    }
}

#[tauri::command]
pub fn is_valid_audio_file(path: String) -> Result<bool, String> {
    log::debug!("Invoked is_valid_audio_file function with path: {}", path);

    let p = Path::new(&path);

    if !p.is_file() {
        log::info!("Path is not a file");

        return Ok(false);
    }

    let allowed = ["mp3", "wav", "ogg", "flac", "m4a", "aac", "opus"];

    if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
        let ext = ext.to_lowercase();

        if !allowed.contains(&ext.as_str()) {
            log::info!("File extension not found or not supported");
        }

        return Ok(allowed.contains(&ext.as_str()));
    }

    Ok(false)
}

pub fn build_playlist(current_path: &str) -> Result<usize, String> {
    let path = Path::new(current_path);
    let parent = path.parent().ok_or("Invalid path, no parent directory")?;

    let mut files: Vec<String> = std::fs::read_dir(parent)
        .map_err(|e| e.to_string())?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let p = entry.path();

            if !p.is_file() {
                return None;
            }

            let str = p.to_string_lossy().to_string();

            match is_valid_audio_file(str.clone()) {
                Ok(true) => Some(str),
                _ => None,
            }
        })
        .collect();

    files.sort();

    let current_index = files
        .iter()
        .position(|f| f == current_path)
        .ok_or("Current file not found in playlist")?;

    *PLAYLIST.lock().unwrap() = files;
    *CURRENT_INDEX.lock().unwrap() = current_index;

    Ok(current_index)
}

pub fn stop_current() {
    if let Some(sink) = PLAYER.lock().unwrap().take() {
        sink.stop();
    }
}

pub fn handle_initial_argv(app: &AppHandle) {
    if let Some(path) = std::env::args().nth(1) {
        if let Some(win) = app.get_webview_window("main") {
            let _ = win.emit("open-file", path);
        }
    }
}
