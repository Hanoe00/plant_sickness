use wasm_bindgen::prelude::*;
use image::{load_from_memory, ImageFormat};
use std::io::Cursor;
use base64::{engine::general_purpose, Engine as _};

#[wasm_bindgen]
pub fn process_image_bytes(image_bytes: &[u8]) -> Result<String, JsValue> {
    // Decodeing raw image byte array
    let img = load_from_memory(image_bytes)
        .map_err(|e| JsValue::from_str(&format!("Failed to decode image: {}", e)))?;

    // Resize image to 224x224 (standard ML input size)
    let resized = img.resize_exact(224, 224, image::imageops::FilterType::Triangle);

    //Writing encoded PNG bytes into memory
    let mut encoded_bytes = Vec::new();
    let mut cursor = Cursor::new(&mut encoded_bytes);
    resized.write_to(&mut cursor, ImageFormat::Png)
        .map_err(|e| JsValue::from_str(&format!("Failed to encode image: {}", e)))?;

    //Returning as Data URL string to render image preview in JS
    let base64_str = general_purpose::STANDARD.encode(&encoded_bytes);
    Ok(format!("data:image/png;base64,{}", base64_str))
}