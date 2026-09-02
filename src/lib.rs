use wasm_bindgen::prelude::*;
use image::load_from_memory;

#[wasm_bindgen]
pub fn process_to_rgba(image_bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
    // 1. Decode raw image
    let img = load_from_memory(image_bytes)
        .map_err(|e| JsValue::from_str(&format!("Failed to decode image: {}", e)))?;

    // 2. Resize to 224x224
    let resized = img.resize_exact(224, 224, image::imageops::FilterType::Triangle);

    // 3. Convert to RGBA 8-bit image and extract raw pixel buffer
    let rgba_img = resized.to_rgba8();
    let raw_pixels: Vec<u8> = rgba_img.into_raw();

    // 4. Return raw byte slice directly to JS Canvas ImageData
    Ok(raw_pixels)
}