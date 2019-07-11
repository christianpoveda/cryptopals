use crate::{Decoder, Encoder};

pub struct Base16Encoder;

impl Encoder for Base16Encoder {
    fn encode(&self, bytes: &[u8]) -> String {
        bytes
            .iter()
            .map(|&byte| std::char::from_digit(byte as u32, 16).unwrap())
            .collect()
    }
}

pub struct Base16Decoder;

impl Decoder for Base16Decoder {
    fn decode(&self, string: &str) -> Vec<u8> {
        string
            .chars()
            .map(|chr| chr.to_digit(16).unwrap() as u8)
            .collect()
    }
}
