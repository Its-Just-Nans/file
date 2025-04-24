use serde_json::{Map, Value};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::Output;

#[wasm_bindgen]
pub fn process_file(data: &[u8], filename: &str, source_url: &str) {
    let output = crate::process_file_raw(data, filename);
    let mut map: Map<String, Value> = serde_json::to_value(&output)
        .unwrap()
        .as_object()
        .unwrap()
        .clone();
    if output.file_len > 1000 {
        map.insert(
            "file_len_KiB".into(),
            Value::String(format!("{:.2} KiB (1024)", output.file_len / 1024)),
        );
        map.insert(
            "file_len_kB".into(),
            Value::String(format!("{:.2} kB (1000)", output.file_len / 1000)),
        );
        if output.file_len > 1000000 {
            map.insert(
                "file_len_MiB".into(),
                Value::String(format!(
                    "{:.2} MiB (1024²)",
                    output.file_len / (1024 * 1024)
                )),
            );
            map.insert(
                "file_len_MB".into(),
                Value::String(format!("{:.2} MB (1000²)", output.file_len / (1000 * 1000))),
            );
        }
    }
    post_preview(&output, &map, source_url);
}

fn create_image_element(source_url: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    let img = document.create_element("img").unwrap();
    img.set_attribute("src", source_url).unwrap();
    let details3 = document.create_element("details").unwrap();
    let summary3 = document.create_element("summary").unwrap();
    summary3.set_inner_html("Image");
    details3.append_child(&summary3).unwrap();
    details3.append_child(&img).unwrap();
    document
        .get_element_by_id("dropHandler")
        .unwrap()
        .append_child(&details3)
        .unwrap();
}

fn create_audio_element(source_url: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    let audio = document.create_element("audio").unwrap();
    audio.set_attribute("controls", "true").unwrap();
    audio.set_attribute("src", source_url).unwrap();
    let details3 = document.create_element("details").unwrap();
    let summary3 = document.create_element("summary").unwrap();
    summary3.set_inner_html("Audio");
    details3.append_child(&summary3).unwrap();
    details3.append_child(&audio).unwrap();
    document
        .get_element_by_id("dropHandler")
        .unwrap()
        .append_child(&details3)
        .unwrap();
}

fn create_video_element(source_url: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    let video = document.create_element("video").unwrap();
    video.set_attribute("controls", "true").unwrap();
    video.set_attribute("src", source_url).unwrap();
    let details3 = document.create_element("details").unwrap();
    let summary3 = document.create_element("summary").unwrap();
    summary3.set_inner_html("Video");
    details3.append_child(&summary3).unwrap();
    details3.append_child(&video).unwrap();
    document
        .get_element_by_id("dropHandler")
        .unwrap()
        .append_child(&details3)
        .unwrap();
}

fn create_font_element(source_url: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    let text = document.create_element("p").unwrap();
    let font_name = "UploadedFont";
    text.set_attribute("id", "font").unwrap();
    text.set_inner_html("The fox jumps over the lazy dog");
    text.set_attribute("style", "font-size: 2em; font-family: UploadedFont")
        .unwrap();
    let style = document.create_element("style").unwrap();
    style.set_inner_html(&format!(
        "@font-face {{ font-family: \"{}\"; src: url('{}'); }}",
        font_name, source_url
    ));
    let details3 = document.create_element("details").unwrap();
    let summary3 = document.create_element("summary").unwrap();
    summary3.set_inner_html("Font");
    details3.append_child(&summary3).unwrap();
    details3.append_child(&style).unwrap();
    details3.append_child(&text).unwrap();
    document
        .get_element_by_id("dropHandler")
        .unwrap()
        .append_child(&details3)
        .unwrap();
}

pub fn post_preview_file(kind: &str, source_url: &str) {
    if kind == "Image" {
        create_image_element(source_url);
    } else if kind == "Audio" {
        create_audio_element(source_url);
    } else if kind == "Video" {
        create_video_element(source_url);
    } else if kind == "Font" {
        create_font_element(source_url);
    }
}

fn create_view_as_string() {
    let document = web_sys::window().unwrap().document().unwrap();
    let details = document.create_element("details").unwrap();
    let summary = document.create_element("summary").unwrap();
    summary.set_inner_html("as string");
    summary.set_id("string-summary");
    details.append_child(&summary).unwrap();
    let pre = document.create_element("pre").unwrap();
    pre.set_id("string");
    pre.set_inner_html("Loading...");
    details.append_child(&pre).unwrap();
    document
        .get_element_by_id("dropHandler")
        .unwrap()
        .append_child(&details)
        .unwrap();
}

fn create_view_as_binary() {
    let document = web_sys::window().unwrap().document().unwrap();
    let details = document.create_element("details").unwrap();
    let summary = document.create_element("summary").unwrap();
    summary.set_inner_html("as [u8]");
    summary.set_id("binary-summary");
    details.append_child(&summary).unwrap();
    let pre = document.create_element("pre").unwrap();
    pre.set_id("binary");
    pre.set_inner_html("Loading...");
    details.append_child(&pre).unwrap();
    document
        .get_element_by_id("dropHandler")
        .unwrap()
        .append_child(&details)
        .unwrap();
}

fn create_view_as_hex() {
    let document = web_sys::window().unwrap().document().unwrap();
    let details = document.create_element("details").unwrap();
    let summary = document.create_element("summary").unwrap();
    summary.set_inner_html("as hex");
    summary.set_id("hex-summary");
    details.append_child(&summary).unwrap();
    let pre = document.create_element("pre").unwrap();
    pre.set_id("hex");
    pre.set_inner_html("Loading...");
    details.append_child(&pre).unwrap();
    document
        .get_element_by_id("dropHandler")
        .unwrap()
        .append_child(&details)
        .unwrap();
}

pub fn post_preview(output: &Output, output_map: &Map<String, Value>, source_url: &str) {
    let json_str = match serde_json::to_string_pretty(&output_map) {
        Ok(json) => json,
        Err(_) => "{\"error\": \"Error serializing output\"}".to_string(),
    };
    let document = web_sys::window().unwrap().document().unwrap();
    let drop_handler = document.get_element_by_id("dropHandler").unwrap();

    document.get_element_by_id("startDiv").unwrap().remove();

    let div = document.create_element("div").unwrap();
    div.set_id("infos");
    div.set_inner_html(&json_str);
    drop_handler.append_child(&div).unwrap();

    create_view_as_string();
    create_view_as_binary();
    create_view_as_hex();

    post_preview_file(&output.file_type.kind, source_url);
}
