use file_format::FileFormat;
use serde::Serialize;
use serde_json::{Map, Value};
use std::io::BufReader;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Serialize)]
pub struct FileFormatDetected {
    kind: String,
    format_name: String,
    format_short_name: String,
    media_type: String,
    extension: String,
    extra_data: Value,
}

#[derive(Serialize)]
pub struct Output {
    file_path: String,
    file_len: usize,
    file_type: FileFormatDetected,
}

fn get_exif(data: &[u8]) -> Value {
    let exifreader = exif::Reader::new();
    let mut reader = std::io::Cursor::new(data);
    let exif = exifreader.read_from_container(&mut reader);
    let exif = match exif {
        Ok(exif) => exif,
        Err(_) => return Value::Array(Vec::new()),
    };
    let mut exif_fields: Vec<Value> = Vec::new();
    for f in exif.fields() {
        let mut value = Map::new();
        value.insert("tag".to_string(), f.tag.to_string().into());
        value.insert("ifd_num".to_string(), f.ifd_num.to_string().into());
        value.insert(
            "display_value".to_string(),
            f.display_value().to_string().into(),
        );
        exif_fields.push(value.into());
    }
    exif_fields.into()
}

fn get_zip(data: &[u8]) -> Value {
    let reader = std::io::Cursor::new(data);
    let reader = BufReader::new(reader);

    let mut archive = match zip::ZipArchive::new(reader) {
        Ok(archive) => archive,
        Err(err) => {
            let mut object: Map<String, Value> = Map::new();
            object.insert("error".to_string(), err.to_string().into());
            return object.into();
        }
    };

    let mut object: Map<String, Value> = Map::new();

    object.insert("archive_len".to_string(), archive.len().into());
    let mut entries: Vec<Value> = Vec::new();
    for i in 0..archive.len() {
        let file = match archive.by_index(i) {
            Ok(file) => file,
            Err(err) => {
                entries.push(format!("Error reading entry {}: {}", i, err).into());
                continue;
            }
        };
        let mut entry_map = Map::new();
        let outpath = match file.enclosed_name() {
            Some(path) => path,
            None => {
                entry_map.insert("path".to_string(), file.name().into());
                entry_map.insert("suspicious_path".to_string(), true.into());
                continue;
            }
        };
        entry_map.insert("path".to_string(), format!("{}", outpath.display()).into());
        entry_map.insert("name".to_string(), file.name().into());
        entry_map.insert("size".to_string(), file.size().into());
        entry_map.insert("compressed_size".to_string(), file.compressed_size().into());
        entry_map.insert(
            "compression".to_string(),
            file.compression().to_string().into(),
        );
        entry_map.insert("encrypted".to_string(), file.encrypted().into());
        if let Some(mode) = file.unix_mode() {
            entry_map.insert("unix_mode".to_string(), mode.into());
        }
        if let Some(last_modified) = file.last_modified() {
            entry_map.insert(
                "last_modified".to_string(),
                last_modified.to_string().into(),
            );
        }

        let comment = file.comment();
        if !comment.is_empty() {
            entry_map.insert("comment".to_string(), comment.into());
        }

        if file.is_dir() {
            entry_map.insert("type".to_string(), "directory".into());
        } else {
            entry_map.insert("type".to_string(), "file".into());
        }
        entries.push(entry_map.into());
    }
    object.insert("entries".to_string(), entries.into());
    object.into()
}

pub fn process_file_raw(data: &[u8], file_path: &str) -> Output {
    let file_len = data.len();
    let fmt = FileFormat::from_bytes(data);
    let extra_data = match fmt.kind() {
        file_format::Kind::Image => get_exif(data),
        file_format::Kind::Archive => get_zip(data),
        _ => {
            let vec: Vec<()> = Vec::new();
            vec.into()
        }
    };
    let output = Output {
        file_path: file_path.to_string(),
        file_len,
        file_type: FileFormatDetected {
            kind: format!("{:?}", fmt.kind()),
            format_name: fmt.name().to_string(),
            format_short_name: fmt
                .short_name()
                .map(|s| s.to_string())
                .unwrap_or("".to_string()),
            media_type: fmt.media_type().to_string(),
            extension: fmt.extension().to_string(),
            extra_data,
        },
    };
    output
}

#[wasm_bindgen]
pub fn process_file(data: &[u8], filename: &str) -> String {
    let output = process_file_raw(data, filename);
    match serde_json::to_string(&output) {
        Ok(json) => json,
        Err(_) => "{\"error\": \"Error serializing output\"}".to_string(),
    }
}

pub fn get_size(size: usize, human: bool, iec: bool) -> String {
    if !human {
        return size.to_string();
    }
    let mut size = size as f64;
    let (units, changer) = if iec {
        (["B", "KiB", "MiB", "GiB", "TiB"], 1024.0)
    } else {
        (["B", "kB", "MB", "GB", "TB"], 1000.0)
    };
    let mut i = 0;
    while size >= changer && i < units.len() - 1 {
        size /= changer;
        i += 1;
    }
    let size = format!("{:.2}", size);
    let size = size.trim_end_matches('0').trim_end_matches('.');
    if size.is_empty() {
        return "0".to_string();
    }
    format!("{} {}", size, units[i])
}

pub mod cli;
pub use cli::cli_main;
