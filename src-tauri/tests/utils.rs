use sound_player_lib::{build_playlist, get_audio_cover, is_valid_audio_file};

#[test]
fn test_is_valid_audio_file_command() {
    let path = "tests/assets/audio/sample.mp3".to_string();
    let path_error = "tests/assets/sample.txt".to_string();

    assert_eq!(is_valid_audio_file(path), Ok(true));
    assert_eq!(is_valid_audio_file(path_error), Ok(false));
}

#[test]
fn test_get_audio_cover_command() {
    let cover_path = "tests/assets/audio/sample_cover.wav".to_string();
    let cover_result = get_audio_cover(cover_path);

    match cover_result {
        Ok(opt_cover) => {
            assert!(opt_cover.is_some(), "Expected cover for file with cover");
        }
        Err(e) => panic!("Failed to get cover from file with cover: {}", e),
    }
}

#[test]
fn test_not_get_audio_cover_command() {
    let no_cover_path = "tests/assets/audio/sample.mp3".to_string();
    let no_cover_result = get_audio_cover(no_cover_path);

    match no_cover_result {
        Ok(opt_cover) => {
            assert!(
                opt_cover.is_none(),
                "Expected no cover for file without cover"
            );
        }
        Err(e) => panic!("Unexpected error on no-cover file: {}", e),
    }
}

#[test]
fn test_build_playlist() {
    let path = "tests/assets/audio/sample.mp3";
    let result = build_playlist(path);

    assert!(result.is_ok(), "Failed to build playlist");
}
