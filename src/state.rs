use std::collections::HashMap;
use std::sync::OnceLock;

static ENV: OnceLock<HashMap<String, String>> = OnceLock::new();

pub fn init(map: HashMap<String, String>) -> bool {
    ENV.set(map).is_ok()
}

pub fn get(key: &str) -> Option<&'static str> {
    ENV.get().and_then(|m| m.get(key)).map(|s| s.as_str())
}

pub fn is_initialized() -> bool {
    ENV.get().is_some()
}

pub fn count() -> usize {
    ENV.get().map_or(0, |m| m.len())
}
