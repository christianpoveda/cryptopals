pub struct Encoder;

impl crate::Encoder for Encoder {
    fn encode(&self, bytes: &[u8]) -> String {
        bytes
            .iter()
            .map(|&byte| std::char::from_digit(byte as u32, 16).unwrap())
            .collect()
    }
}

pub struct Decoder;

impl crate::Decoder for Decoder {
    fn decode(&self, string: &str) -> Vec<u8> {
        string
            .chars()
            .map(|chr| chr.to_digit(16).unwrap() as u8)
            .collect()
    }
}
