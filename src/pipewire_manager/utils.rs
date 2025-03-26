use libspa::utils::dict::DictRef;

pub const UNKNOWN_STR: &str = "unknown";

pub fn val(dict: &DictRef, key: &str) -> String {
    dict.get(key)
        .expect(&format!("Expected key {key} does not exist."))
        .to_string()
}

pub fn val_or(dict: &DictRef, key: &str, default: &str) -> String {
    dict.get(key).unwrap_or(default).to_string()
}

pub fn val_opt(dict: &DictRef, key: &str) -> Option<String> {
    dict.get(key).map(|s| s.to_string())
}
