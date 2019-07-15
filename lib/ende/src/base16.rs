use lazy_static::lazy_static;
use std::collections::HashMap;

const CHARS: &'static [u8] = b"0123456789abcdef";
lazy_static! {
    static ref BYTES: HashMap<u8, u8> = CHARS
        .iter()
        .enumerate()
        .map(|(i, &b)| (b, i as u8))
        .collect();
}

pub struct Encoder;

impl crate::Encoder for Encoder {
    fn encode(&self, bytes: &[u8]) -> String {
        let mut utf8_bytes = Vec::new();
        for &byte in bytes {
            let byte = byte as usize;
            utf8_bytes.push(CHARS[byte / 16]);
            utf8_bytes.push(CHARS[byte % 16]);
        }
        String::from_utf8(utf8_bytes).unwrap()
    }
}

pub struct Decoder;

impl crate::Decoder for Decoder {
    fn decode(&self, string: &str) -> Vec<u8> {
        string
            .as_bytes()
            .chunks(2)
            .map(|bytes| match bytes {
                [a, b] => BYTES[a] * 16 + BYTES[b],
                _ => unreachable!(),
            })
            .collect()
    }
}
