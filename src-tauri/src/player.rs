use crate::{build_playlist, stop_current, AUDIO, CURRENT_INDEX, LAST_PATH, PLAYER, PLAYLIST};

use rodio::{Decoder, Sink};
use std::fs::File;
use std::io::BufReader;

#[tauri::command]
pub fn play_audio(path: String) -> Result<(), String> {
    log::debug!("Invoked play_audio function with path: {}", path);

    stop_current();

    let need_init = {
        let pl = PLAYLIST.lock().unwrap();

        pl.is_empty() || !pl.iter().any(|p| *p == path)
    };

    if need_init {
        build_playlist(&path)?;
    } else {
        let mut idx_guard = CURRENT_INDEX.lock().unwrap();
        let pl = PLAYLIST.lock().unwrap();

        if let Some(i) = pl.iter().position(|p| *p == path) {
            *idx_guard = i;
        }
    }

    let file = File::open(&path).map_err(|e| e.to_string())?;
    let source = Decoder::new(BufReader::new(file)).map_err(|e| e.to_string())?;
    let sink = Sink::try_new(&*AUDIO).map_err(|e| e.to_string())?;

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

#[tauri::command]
pub fn next_audio() -> Result<(), String> {
    log::debug!("Invoked next_audio function");

    let path = {
        let playlist = PLAYLIST.lock().unwrap();

        if playlist.is_empty() {
            log::error!("Playlist is empty");
            return Err("Playlist is empty".into());
        }

        let mut index = CURRENT_INDEX.lock().unwrap();

        *index = (*index + 1) % playlist.len();

        playlist[*index].clone()
    };

    log::info!("Audio playback moved forward successfully");

    play_audio(path)
}

#[tauri::command]
pub fn previous_audio() -> Result<(), String> {
    log::debug!("Invoked previous_audio function");

    let path = {
        let playlist = PLAYLIST.lock().unwrap();

        if playlist.is_empty() {
            log::error!("Playlist is empty");
            return Err("Playlist is empty".into());
        }

        let mut index = CURRENT_INDEX.lock().unwrap();

        *index = if *index == 0 {
            playlist.len() - 1
        } else {
            *index - 1
        };

        playlist[*index].clone()
    };

    log::info!("Audio playback moved backward successfully");

    play_audio(path)
}

#[tauri::command]
pub fn get_current_audio() -> Option<String> {
    log::debug!("Invoked get_current_audio function");

    let playlist = PLAYLIST.lock().unwrap();
    let index = *CURRENT_INDEX.lock().unwrap();

    log::info!("Current audio got successfully");

    playlist.get(index).cloned()
}
