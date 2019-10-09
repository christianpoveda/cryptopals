use std::collections::HashMap;

pub mod lang;

pub fn bytes_freq(data: &[u8]) -> HashMap<u8, f64> {
    let mut map = HashMap::new();
    let total = data.len() as f64;
    for &byte in data {
        *map.entry(byte).or_insert(0) += 1;
    }

    map.into_iter()
        .map(|(k, v)| (k, v as f64 / total))
        .collect()
}
