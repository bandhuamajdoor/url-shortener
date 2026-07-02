use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct URLmap {
    pub longurl: String,
    pub code: String,
}

fn ensure_parent_dir(file_path: &str) {
    if let Some(parent) = Path::new(file_path).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).ok();
        }
    }
}

pub fn load_url_maps(file_path: &str) -> Vec<URLmap> {
    let content = fs::read_to_string(file_path).unwrap_or_default();

    if content.trim().is_empty() {
        return Vec::new();
    }

    serde_json::from_str(&content).unwrap_or_default()
}

pub fn save_url_map(file_path: &str, entry: URLmap) -> std::io::Result<()> {
    ensure_parent_dir(file_path);

    let mut entries = load_url_maps(file_path);
    entries.push(entry);

    let updated_json = serde_json::to_string_pretty(&entries).unwrap();
    fs::write(file_path, updated_json)
}

pub fn find_long_url_for_code(file_path: &str, code: &str) -> Option<String> {
    let normalized_code = normalize_code(code);

    load_url_maps(file_path)
        .into_iter()
        .find(|entry| normalize_code(&entry.code) == normalized_code)
        .map(|entry| entry.longurl)
}

pub fn list_url_maps(file_path: &str) -> Vec<URLmap> {
    load_url_maps(file_path)
}

pub fn normalize_code(code: &str) -> String {
    code.trim().trim_start_matches('/').to_string()
}
