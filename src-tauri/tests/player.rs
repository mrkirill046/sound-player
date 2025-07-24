use sound_player_lib::{pause_audio, play_audio, restart_audio, resume_audio};

const SAMPLE: &str = "tests/assets/sample.mp3";

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
