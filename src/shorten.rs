use rand::distr::{Alphanumeric,SampleString};
use std::collections::HashMap;
use url::form_urlencoded;

pub fn generate_short_code() -> String {
    Alphanumeric.sample_string(&mut rand::rng(), 6)
}

pub fn decode_url_form_body(body: &str) -> String {
    let parsed: HashMap<String, String> = form_urlencoded::parse(body.as_bytes())
        .into_owned()
        .collect();

    match parsed.get("url") {
        Some(url) if !url.is_empty() => url.clone(),
        _ => {
            eprintln!("Error: 'longurl' key not found or empty in the body: {}", body);
            String::new()
        }
    }
}
