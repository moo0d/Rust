use qrcode::QrCode;
use image::Luma;
use sha2::{Sha256, Digest};
use std::path::{Path};

pub fn generate_qr_code(input: &str, output_dir: &str) -> Result<String, Box<dyn std::error::Error>> {
    if input.is_empty() {
        return Err("Input string is empty".into());
    }

    if input.chars().any(|c| c.is_control() && c != '\n' && c != '\t') {
        return Err("Input string contains invalid characters".into());
    }

    let code = QrCode::new(input.as_bytes())?;
    let image = code.render::<Luma<u8>>().build();

    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let hash = hasher.finalize();
    let filename = format!("{:x}.png", hash);

    let path = Path::new(output_dir).join(filename);
    image.save(&path)?;

    Ok(path.to_string_lossy().into_owned())
}