use base64::engine::general_purpose;
use base64::Engine as _;
use lofty::{read_from_path, PictureType, TaggedFileExt};

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
