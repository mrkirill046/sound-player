use once_cell::sync::Lazy;
use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::sync::Mutex;

pub static AUDIO: Lazy<OutputStreamHandle> = Lazy::new(|| {
    let (stream, handle) = OutputStream::try_default().expect("Failed to create stream");

    Box::leak(Box::new(stream));

    handle
});

pub static PLAYER: Lazy<Mutex<Option<Sink>>> = Lazy::new(|| Mutex::new(None));
pub static LAST_PATH: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));
pub static PLAYLIST: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static CURRENT_INDEX: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));
