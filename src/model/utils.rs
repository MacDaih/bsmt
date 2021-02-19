use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_time() -> u128 {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
pub fn fmt_zeros(level: usize) -> String {
    let mut result = String::new();
    for _i in 0..level {
        result.push_str("0");
    }
    result
}
pub fn calc_hash(value: String) -> String {
    let b_arr = value
        .as_bytes()
        .to_owned();
    let mut hasher = Sha256::default();
    hasher.update(&b_arr);
    let mut result = String::new();
    let output = hasher.finalize();
    for o in output {
        let hex = String::from(format!("{:x}",o));
        if hex.len() == 1 {
            result.push_str("0");
        }
        result.push_str(&hex);
    }
    return result;
}