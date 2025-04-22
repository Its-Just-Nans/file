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
    is_img: bool,
    exif_data: Vec<String>,
}

#[derive(Serialize)]
struct Output {
    file_len: usize,
    file_type: FileFormatDetected,
}

fn get_exif(data: &[u8]) -> Vec<String> {
    let exifreader = exif::Reader::new();
    let mut reader = std::io::Cursor::new(data);
    let exif = exifreader.read_from_container(&mut reader);
    let exif = match exif {
        Ok(exif) => exif,
        Err(_) => return Vec::new(),
    };
    let mut exif_fields = Vec::new();
    for f in exif.fields() {
        exif_fields.push(format!(
            "{} {} {}",
            f.tag,
            f.ifd_num,
            f.display_value().with_unit(&exif)
        ));
    }
    exif_fields
}

#[wasm_bindgen]
pub fn process_file(data: &[u8]) -> String {
    let file_len = data.len();
    let fmt = FileFormat::from_bytes(data);
    let is_img = fmt.kind() == file_format::Kind::Image;
    let exif_data = if is_img { get_exif(data) } else { Vec::new() };
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
            is_img,
            exif_data,
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
