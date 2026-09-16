use burn_import::onnx::ModelGen;
use std::path::PathBuf;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let onnx_path = PathBuf::from(&manifest_dir).join("model/plant_disease_model.onnx");
    let out_dir = PathBuf::from(&manifest_dir).join("src");

    ModelGen::new()
        .input(onnx_path.to_str().unwrap())
        .out_dir(out_dir.to_str().unwrap())
        .run_from_script();
}