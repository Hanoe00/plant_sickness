use wasm_bindgen::prelude::*;
use image::{DynamicImage, RgbaImage, ImageReader};
use std::io::Cursor;
use exif;

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

/// Filter options.
#[wasm_bindgen]
pub struct FilterOptions {
    pub contrast: f32,    // Range: -100.0 to 100.0 (0.0 = default)
    pub brightness: i32,  // Range: -255 to 255 (0 = default)
    pub blur_sigma: f32,  // Range: 0.0 to 10.0 (0.0 = no blur)
    pub grayscale: bool,  // Convert to grayscale
}

#[wasm_bindgen]
impl FilterOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(contrast: f32, brightness: i32, blur_sigma: f32, grayscale: bool) -> Self {
        Self {
            contrast,
            brightness,
            blur_sigma,
            grayscale,
        }
    }
}

/// Pipeline:
/// Decoding -> EXIF Correction -> Filtering -> Resizing (224x224) -> Leaf Segmentation (HSV) -> Normalization
#[wasm_bindgen]
pub fn process_image_full(
    image_bytes: &[u8],
    filters: Option<FilterOptions>,
) -> Result<ProcessedResult, JsValue> {
    // decoding and exif
    let reader = ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format()
        .map_err(|e| JsValue::from_str(&format!("Failed to guess image format: {}", e)))?;

    let mut img = reader
        .decode()
        .map_err(|e| JsValue::from_str(&format!("Failed to decode image bytes: {}", e)))?;

    img = apply_exif_orientation(image_bytes, img);

    // adding filters to img
    if let Some(opts) = filters {
        img = apply_image_filters(img, &opts);
    }

    // image scaling
    let resized = img.resize_exact(224, 224, image::imageops::FilterType::Lanczos3);
    let mut rgba_img = resized.to_rgba8();

    // leaf segmentation
    segment_leaf_hsv(&mut rgba_img);

    // normalisation of rgb 
    let mut normalized_tensor = Vec::with_capacity(224 * 224 * 3);
    for pixel in rgba_img.pixels() {
        normalized_tensor.push((pixel[0] as f32) / 255.0);
        normalized_tensor.push((pixel[1] as f32) / 255.0);
        normalized_tensor.push((pixel[2] as f32) / 255.0);
    }

    Ok(ProcessedResult {
        rgba_bytes: rgba_img.into_raw(),
        normalized_tensor,
    })
}

/// Applies image adjustments (Contrast, Brightness, Blur, Grayscale)
fn apply_image_filters(mut img: DynamicImage, filters: &FilterOptions) -> DynamicImage {
    if filters.grayscale {
        img = img.grayscale();
    }
    if filters.contrast != 0.0 {
        img = img.adjust_contrast(filters.contrast);
    }
    if filters.brightness != 0 {
        img = img.brighten(filters.brightness);
    }
    if filters.blur_sigma > 0.0 {
        img = img.blur(filters.blur_sigma);
    }
    img
}

/// HSV Thresholding for leaf region separation
fn segment_leaf_hsv(img: &mut RgbaImage) {
    for pixel in img.pixels_mut() {
        let r = pixel[0] as f32 / 255.0;
        let g = pixel[1] as f32 / 255.0;
        let b = pixel[2] as f32 / 255.0;

        let (h, s, v) = rgb_to_hsv(r, g, b);

        let is_leaf = (h >= 25.0 && h <= 160.0) && (s >= 0.15) && (v >= 0.15);

        if !is_leaf {
            pixel[0] = (pixel[0] as f32 * 0.2) as u8;
            pixel[1] = (pixel[1] as f32 * 0.2) as u8;
            pixel[2] = (pixel[2] as f32 * 0.2) as u8;
        }
    }
}
// transition to hsv
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
// exif corection
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

//tests
#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};

    /// Generates mock PNG byte stream for testing
    fn create_dummy_png_bytes(width: u32, height: u32) -> Vec<u8> {
        let img = ImageBuffer::from_fn(width, height, |x, _y| {
            if x % 2 == 0 {
                Rgb([0u8, 200u8, 0u8]) // Green pixel (leaf)
            } else {
                Rgb([200u8, 0u8, 0u8]) // Red pixel (non-leaf)
            }
        });

        let mut bytes: Vec<u8> = Vec::new();
        let mut cursor = Cursor::new(&mut bytes);
        DynamicImage::ImageRgb8(img)
            .write_to(&mut cursor, image::ImageFormat::Png)
            .unwrap();
        bytes
    }

    #[test]
    fn test_rgb_to_hsv_primary_colors() {
        // Pure Red
        let (h, s, v) = rgb_to_hsv(1.0, 0.0, 0.0);
        assert_eq!(h, 0.0);
        assert_eq!(s, 1.0);
        assert_eq!(v, 1.0);

        // Pure Green
        let (h, s, v) = rgb_to_hsv(0.0, 1.0, 0.0);
        assert_eq!(h, 120.0);
        assert_eq!(s, 1.0);
        assert_eq!(v, 1.0);

        // Pure Blue
        let (h, s, v) = rgb_to_hsv(0.0, 0.0, 1.0);
        assert_eq!(h, 240.0);
        assert_eq!(s, 1.0);
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_process_image_full_output_dimensions() {
        let png_bytes = create_dummy_png_bytes(100, 100);
        let result = process_image_full(&png_bytes, None);

        assert!(result.is_ok());
        let res = result.unwrap();

        // 224 x 224 pixels with RGBA (4 channels per pixel)
        assert_eq!(res.rgba_bytes().len(), 224 * 224 * 4);

        // 224 x 224 pixels with RGB (3 float channels per pixel)
        assert_eq!(res.normalized_tensor().len(), 224 * 224 * 3);
    }

    #[test]
    fn test_process_image_full_with_filters() {
        let png_bytes = create_dummy_png_bytes(50, 50);
        let filters = FilterOptions::new(10.0, 5, 1.0, true);

        let result = process_image_full(&png_bytes, Some(filters));

        assert!(result.is_ok());
        let res = result.unwrap();
        assert_eq!(res.normalized_tensor().len(), 224 * 224 * 3);
    }

    #[test]
    fn test_leaf_segmentation_hsv() {
        let mut img = RgbaImage::new(1, 2);

        // Pixel 0: Green leaf-like color
        img.put_pixel(0, 0, image::Rgba([0, 200, 0, 255]));
        // Pixel 1: Red background color
        img.put_pixel(0, 1, image::Rgba([200, 0, 0, 255]));

        segment_leaf_hsv(&mut img);

        // Green pixel should remain unchanged
        assert_eq!(img.get_pixel(0, 0)[1], 200);

        // Red pixel should be darkened (200 * 0.2 = 40)
        assert_eq!(img.get_pixel(0, 1)[0], 40);
    }
}