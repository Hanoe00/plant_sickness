use wasm_bindgen::prelude::*;
use image::{DynamicImage, RgbaImage, ImageReader};
use std::io::Cursor;
use exif;

use burn::tensor::Tensor;
use burn_ndarray::NdArray;

// Implementacja wygenerowanego z ONNX modelu Burn
mod model {
    pub mod generated {
        include!(concat!(env!("OUT_DIR"), "/model/plant_disease_model.rs"));
    }
}

type Backend = NdArray<f32>;

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

/// Opcje filtrowania obrazu
#[wasm_bindgen]
pub struct FilterOptions {
    pub contrast: f32,    // Zakres: -100.0 do 100.0 (0.0 = domyślne)
    pub brightness: i32,  // Zakres: -255 do 255 (0 = domyślne)
    pub blur_sigma: f32,  // Zakres: 0.0 do 10.0 (0.0 = brak rozmycia)
    pub grayscale: bool,  // Konwersja do odcieni szarości
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

/// Pipeline wstępnego przetwarzania obrazu:
/// Dekodowanie -> Korekcja EXIF -> Filtrowanie -> Skalowanie (224x224) -> Segmentacja liścia (HSV) -> Normalizacja ImageNet
#[wasm_bindgen]
pub fn process_image_full(
    image_bytes: &[u8],
    filters: Option<FilterOptions>,
) -> Result<ProcessedResult, JsValue> {
    // 1. Dekodowanie obrazu
    let reader = ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format()
        .map_err(|e| JsValue::from_str(&format!("Failed to guess image format: {}", e)))?;

    let mut img = reader
        .decode()
        .map_err(|e| JsValue::from_str(&format!("Failed to decode image bytes: {}", e)))?;

    // 2. Korekcja EXIF
    img = apply_exif_orientation(image_bytes, img);

    // 3. Filtry
    if let Some(opts) = filters {
        img = apply_image_filters(img, &opts);
    }

    // 4. Skalowanie do wymiarów wejściowych sieci (224x224)
    let resized = img.resize_exact(224, 224, image::imageops::FilterType::Lanczos3);
    let mut rgba_img = resized.to_rgba8();

    // 5. Segmentacja liścia w przestrzeni HSV
    segment_leaf_hsv(&mut rgba_img);

    // 6. Normalizacja i ułożenie w formacie NCHW (Planar RGB z uwzględnieniem ImageNet mean/std)
    let mean = [0.485f32, 0.456, 0.406];
    let std = [0.229f32, 0.224, 0.225];
    let mut normalized_tensor = vec![0.0f32; 1 * 3 * 224 * 224];

    for y in 0..224 {
        for x in 0..224 {
            let pixel = rgba_img.get_pixel(x, y);
            let idx = (y * 224 + x) as usize;

            let r = (pixel[0] as f32 / 255.0 - mean[0]) / std[0];
            let g = (pixel[1] as f32 / 255.0 - mean[1]) / std[1];
            let b = (pixel[2] as f32 / 255.0 - mean[2]) / std[2];

            // Układ kanałów CHW: RRR... GGG... BBB...
            normalized_tensor[0 * 224 * 224 + idx] = r;
            normalized_tensor[1 * 224 * 224 + idx] = g;
            normalized_tensor[2 * 224 * 224 + idx] = b;
        }
    }

    Ok(ProcessedResult {
        rgba_bytes: rgba_img.into_raw(),
        normalized_tensor,
    })
}

/// Przekazuje wygenerowany tensor z `process_image_full` bezpośrednio do modelu Burn
#[wasm_bindgen]
pub fn predict_disease(normalized_tensor: &[f32]) -> Result<Vec<f32>, JsValue> {
    if normalized_tensor.len() != 1 * 3 * 224 * 224 {
        return Err(JsValue::from_str(&format!(
            "Invalid tensor length. Expected {}, got {}",
            1 * 3 * 224 * 224,
            normalized_tensor.len()
        )));
    }

    // Inicjalizacja wygenerowanego z ONNX modelu
    let model = model::generated::Model::<Backend>::default();

    // Utworzenie tensora Burn o kształcie [1, 3, 224, 224]
    let tensor_data = normalized_tensor.to_vec();
    let input_tensor = Tensor::<Backend, 4>::from_data(
        burn::tensor::TensorData::new(tensor_data, [1, 3, 224, 224]),
        &Default::default(),
    );

    // Wykonanie inferencji
    let output_logits = model.forward(input_tensor);

    // Konwersja danych wyjściowych za pomocą nowego API Burn (.into_vec::<f32>())
    let logits_vec: Vec<f32> = output_logits
        .into_data()
        .into_vec::<f32>()
        .map_err(|e| JsValue::from_str(&format!("Failed to convert tensor data: {:?}", e)))?;

    Ok(logits_vec)
}

/// Aplikuje modyfikacje obrazu (Kontrast, Jasność, Rozmycie, Odcienie szarości)
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

/// Progowanie HSV w celu wyizolowania obszaru liścia
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

/// Konwersja przestrzeni barw RGB na HSV
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

/// Korekcja obrotu zdjęcia na podstawie metadanych EXIF
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

// --- TESTY ---
#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};

    fn create_dummy_png_bytes(width: u32, height: u32) -> Vec<u8> {
        let img = ImageBuffer::from_fn(width, height, |x, _y| {
            if x % 2 == 0 {
                Rgb([0u8, 200u8, 0u8])
            } else {
                Rgb([200u8, 0u8, 0u8])
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
        let (h, s, v) = rgb_to_hsv(1.0, 0.0, 0.0);
        assert_eq!(h, 0.0);
        assert_eq!(s, 1.0);
        assert_eq!(v, 1.0);

        let (h, s, v) = rgb_to_hsv(0.0, 1.0, 0.0);
        assert_eq!(h, 120.0);
        assert_eq!(s, 1.0);
        assert_eq!(v, 1.0);

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

        assert_eq!(res.rgba_bytes().len(), 224 * 224 * 4);
        assert_eq!(res.normalized_tensor().len(), 1 * 3 * 224 * 224);
    }

    #[test]
    fn test_process_image_full_with_filters() {
        let png_bytes = create_dummy_png_bytes(50, 50);
        let filters = FilterOptions::new(10.0, 5, 1.0, true);

        let result = process_image_full(&png_bytes, Some(filters));

        assert!(result.is_ok());
        let res = result.unwrap();
        assert_eq!(res.normalized_tensor().len(), 1 * 3 * 224 * 224);
    }

    #[test]
    fn test_leaf_segmentation_hsv() {
        let mut img = RgbaImage::new(1, 2);

        img.put_pixel(0, 0, image::Rgba([0, 200, 0, 255]));
        img.put_pixel(0, 1, image::Rgba([200, 0, 0, 255]));

        segment_leaf_hsv(&mut img);

        assert_eq!(img.get_pixel(0, 0)[1], 200);
        assert_eq!(img.get_pixel(0, 1)[0], 40);
    }
}