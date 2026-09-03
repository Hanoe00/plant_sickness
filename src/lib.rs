use wasm_bindgen::prelude::*;
use image::{DynamicImage, RgbaImage, ImageReader};
use std::io::Cursor;
use exif;

/// Represents normalized pixel tensor data ready for neural network input and raw canvas RGBA bytes.
#[wasm_bindgen]
pub struct ProcessedResult {
    rgba_bytes: Vec<u8>,
    normalized_tensor: Vec<f32>,
}

#[wasm_bindgen]
impl ProcessedResult {
    #[wasm_bindgen(getter)]
    pub fn rgba_bytes(&self) -> Vec<u8> {
        self.rgba_bytes.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn normalized_tensor(&self) -> Vec<f32> {
        self.normalized_tensor.clone()
    }
}

/// Full Image Processing Pipeline:
/// Decoding -> EXIF Auto-Orientation -> Resizing (224x224) -> Leaf Segmentation (HSV) -> Normalization
#[wasm_bindgen]
pub fn process_image_full(image_bytes: &[u8]) -> Result<ProcessedResult, JsValue> {
    // 1. DECODING & EXIF CORRECTION
    let reader = ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format()
        .map_err(|e| JsValue::from_str(&format!("Failed to guess image format: {}", e)))?;

    let mut img = reader
        .decode()
        .map_err(|e| JsValue::from_str(&format!("Failed to decode image bytes: {}", e)))?;

    // Apply EXIF orientation correction if present
    img = apply_exif_orientation(image_bytes, img);

    // 2. SCALING (Resize to 224x224 exact resolution)
    let resized = img.resize_exact(224, 224, image::imageops::FilterType::Lanczos3);
    let mut rgba_img = resized.to_rgba8();

    // 3. LEAF SEGMENTATION (HSV Thresholding)
    segment_leaf_hsv(&mut rgba_img);

    // 4. NORMALIZATION (Convert RGBA bytes -> Normalized RGB floats 0.0 to 1.0)
    let mut normalized_tensor = Vec::with_capacity(224 * 224 * 3);
    for pixel in rgba_img.pixels() {
        normalized_tensor.push((pixel[0] as f32) / 255.0); // Red
        normalized_tensor.push((pixel[1] as f32) / 255.0); // Green
        normalized_tensor.push((pixel[2] as f32) / 255.0); // Blue
    }

    // 5. Package output for JavaScript
    Ok(ProcessedResult {
        rgba_bytes: rgba_img.into_raw(),
        normalized_tensor,
    })
}

/// Applies HSV thresholding to retain green and yellow plant tissue while dimming background.
fn segment_leaf_hsv(img: &mut RgbaImage) {
    for pixel in img.pixels_mut() {
        let r = pixel[0] as f32 / 255.0;
        let g = pixel[1] as f32 / 255.0;
        let b = pixel[2] as f32 / 255.0;

        let (h, s, v) = rgb_to_hsv(r, g, b);

        // HSV Leaf Range: Hue between 25° (yellow/brown spots) and 160° (green foliage)
        let is_leaf = (h >= 25.0 && h <= 160.0) && (s >= 0.15) && (v >= 0.15);

        if !is_leaf {
            // Darken non-leaf background pixels
            pixel[0] = (pixel[0] as f32 * 0.2) as u8;
            pixel[1] = (pixel[1] as f32 * 0.2) as u8;
            pixel[2] = (pixel[2] as f32 * 0.2) as u8;
        }
    }
}

/// Helper function to convert RGB [0.0, 1.0] to HSV (Hue: 0-360, Saturation: 0-1, Value: 0-1)
fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let v = max;
    let s = if max == 0.0 { 0.0 } else { delta / max };

    let mut h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    if h < 0.0 {
        h += 360.0;
    }

    (h, s, v)
}

/// Inspects EXIF tag orientation from raw JPEG header bytes using pure-Rust kamadak-exif.
fn apply_exif_orientation(raw_bytes: &[u8], img: DynamicImage) -> DynamicImage {
    let mut cursor = Cursor::new(raw_bytes);
    if let Ok(exif_data) = exif::Reader::new().read_from_container(&mut cursor) {
        if let Some(field) = exif_data.get_field(exif::Tag::Orientation, exif::In::PRIMARY) {
            if let Some(orientation) = field.value.get_uint(0) {
                return match orientation {
                    3 => img.rotate180(),
                    6 => img.rotate90(),
                    8 => img.rotate270(),
                    _ => img,
                };
            }
        }
    }
    img
}