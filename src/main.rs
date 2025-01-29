#![recursion_limit = "1024"]
use console_error_panic_hook::set_once as set_panic_hook;
use file_format::FileFormat;
use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Serialize)]
struct FileFormatDetected {
    kind: String,
    name: String,
    short_name: String,
    media_type: String,
    extension: String,
}

#[derive(Serialize)]
struct Output {
    file_len: usize,
    file_type: FileFormatDetected,
}

#[wasm_bindgen]
pub fn process_file(data: &[u8]) -> String {
    let file_len = data.len();
    let fmt = FileFormat::from_bytes(data);
    let output = Output {
        file_len,
        file_type: FileFormatDetected {
            kind: format!("{:?}", fmt.kind()),
            name: fmt.name().to_string(),
            short_name: fmt
                .short_name()
                .map(|s| s.to_string())
                .unwrap_or("".to_string()),
            media_type: fmt.media_type().to_string(),
            extension: fmt.extension().to_string(),
        },
    };
    match serde_json::to_string(&output) {
        Ok(json) => json,
        Err(_) => "{\"error\": \"Error serializing output\"}".to_string(),
    }
}

fn main() {
    set_panic_hook();
}
