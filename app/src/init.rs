use std::sync::OnceLock;

pub static DEBUG_MODE: OnceLock<bool> = OnceLock::new();
