use sound_player_lib::{
    get_current_audio, next_audio, pause_audio, play_audio, previous_audio, restart_audio,
    resume_audio,
};

const SAMPLE: &str = "tests/assets/audio/sample.mp3";

#[test]
fn test_play_audio_command() {
    play_audio(SAMPLE.into()).expect("Failed to play audio");
}

#[test]
fn test_pause_audio_command() {
    play_audio(SAMPLE.into()).expect("Failed to play audio");
    pause_audio().expect("Failed to pause audio");
}

#[test]
fn test_resume_audio_command() {
    play_audio(SAMPLE.into()).expect("Failed to play audio");
    pause_audio().expect("Failed to pause audio");
    resume_audio().expect("Failed to resume audio");
}

#[test]
fn test_restart_audio_command() {
    play_audio(SAMPLE.into()).expect("Failed to play audio");
    restart_audio().expect("Failed to restart audio");
}

#[test]
fn test_next_audio() {
    play_audio(SAMPLE.into()).expect("Failed to play audio");
    next_audio().expect("Can't go to next audio");
}

#[test]
fn test_previous_audio() {
    play_audio(SAMPLE.into()).expect("Failed to play audio");
    previous_audio().expect("Can't go back to previous audio");
}

#[test]
fn test_get_current_audio() {
    play_audio(SAMPLE.into()).expect("Failed to play audio");

    assert!(get_current_audio().is_some());
}

#[test]
fn test_get_current_audio_empty() {
    assert!(get_current_audio().is_none());
}
