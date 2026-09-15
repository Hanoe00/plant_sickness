use burn_onnx::ModelGen;

fn main() {
    println!("cargo:rerun-if-changed=plant_disease_model.onnx");
    ModelGen::new()
        .input("plant_disease_model.onnx")
        .out_dir("model/")
        .run_from_script();
}