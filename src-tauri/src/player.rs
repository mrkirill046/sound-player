use once_cell::sync::Lazy;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;

static PLAYER: Lazy<Mutex<Option<Sink>>> = Lazy::new(|| Mutex::new(None));
static LAST_PATH: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

#[tauri::command]
pub fn play_audio(path: String) -> Result<(), String> {
    log::debug!("Invoked play_audio function with path: {}", path);

    let (stream, handle) = OutputStream::try_default().map_err(|e| e.to_string())?;
    let _ = Box::leak(Box::new(stream));

    let sink = Sink::try_new(&handle).map_err(|e| e.to_string())?;
    let file = File::open(&path).map_err(|e| e.to_string())?;
    let source = Decoder::new(BufReader::new(file)).map_err(|e| e.to_string())?;

    sink.append(source);
    sink.play();

    *PLAYER.lock().unwrap() = Some(sink);
    *LAST_PATH.lock().unwrap() = Some(path);

    Ok(log::info!("Audio playback started successfully"))
}

#[tauri::command]
pub fn pause_audio() -> Result<(), String> {
    log::debug!("Invoked pause_audio function");

    if let Some(sink) = &*PLAYER.lock().unwrap() {
        sink.pause();
    }

    Ok(log::info!("Audio playback paused successfully"))
}

#[tauri::command]
pub fn resume_audio() -> Result<(), String> {
    log::debug!("Invoked resume_audio function");

    if let Some(sink) = &*PLAYER.lock().unwrap() {
        sink.play();
    }

    Ok(log::info!("Audio playback resumed successfully"))
}

#[tauri::command]
pub fn restart_audio() -> Result<(), String> {
    log::debug!("Invoked restart_audio function");

    let path_opt = LAST_PATH.lock().unwrap().clone();

    if let Some(path) = path_opt {
        play_audio(path)
    } else {
        log::error!("No last track to restart");

        Err("No last track to restart".into())
    }
}
