use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{File, FormData, Request, RequestInit, RequestMode, Response, window};

#[wasm_bindgen]
pub async  fn upload_image(file: File, upload_url: &str) -> Result<String, JsValue>{
    let form_data = FormData::new()?;
    form_data.append_with_blob("file", &file)?;

    // 2. Configure the HTTP POST Request
   let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_body(form_data.as_ref());
    opts.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(upload_url, &opts)?;

    // 3. Send the fetch request via browser window
    let window = window().ok_or_else(|| JsValue::from_str("No window object found"))?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    // 4. Check response status and return the result text
    if resp.ok() {
        let text_promise = resp.text()?;
        let text = JsFuture::from(text_promise).await?;
        Ok(text.as_string().unwrap_or_default())
    } else {
        Err(JsValue::from_str(&format!("Upload failed with status: {}", resp.status())))
    }
}