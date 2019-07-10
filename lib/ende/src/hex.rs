use crate::{Decoder, Encoder};

pub struct HexEncoder;

impl Encoder for HexEncoder {
    fn encode(&self, bytes: &[u8]) -> String {
        bytes
            .iter()
            .map(|&byte| std::char::from_digit(byte as u32, 16).unwrap())
            .collect()
    }
}

pub struct HexDecoder;

impl Decoder for HexDecoder {
    fn decode(&self, string: &str) -> Vec<u8> {
        string
            .chars()
            .map(|chr| chr.to_digit(16).unwrap() as u8)
            .collect()
    }
}
