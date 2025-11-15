use super::{Preview, PreviewContent};
use macroquad::prelude::*;
use std::fs;
use std::path::Path;

pub fn load_preview(preview: &mut Preview, path: &Path, is_dir: bool, size: u64, name: &str) {
    preview.clear();

    if is_dir {
        preview.content = PreviewContent::Info("Dir".into());
        return;
    }

    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "png" | "jpg" | "jpeg" => load_image(preview, path),
        "txt" => load_text(preview, path),
        _ => load_generic_info(preview, size, name),
    }
}

fn load_image(preview: &mut Preview, path: &Path) {
    if let Ok(bytes) = fs::read(path) {
        let tex = Texture2D::from_file_with_format(&bytes, None);
        tex.set_filter(FilterMode::Nearest);
        preview.content = PreviewContent::Image(tex);
    }
}

fn load_text(preview: &mut Preview, path: &Path) {
    if let Ok(text) = fs::read_to_string(path) {
        let preview_text = text.lines().take(20).collect::<Vec<_>>().join("\n");
        preview.content = PreviewContent::Text(preview_text);
    }
}

fn load_generic_info(preview: &mut Preview, size: u64, name: &str) {
    preview.content = PreviewContent::Info(format!("File: {}\n{} bytes", name, size));
}
