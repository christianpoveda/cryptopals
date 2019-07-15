use lazy_static::lazy_static;

use crate::{Decoder, Encoder, EndeError, EndeResult, Table};

const CHARS: Table = Table::Slice(b"0123456789abcdef");
lazy_static! {
    static ref BYTES: Table<'static> = CHARS.transpose();
}

pub struct Base16;

impl Encoder for Base16 {
    fn encode(&self, bytes: &[u8]) -> EndeResult<String> {
        let mut utf8_bytes = Vec::new();

        for &byte in bytes {
            utf8_bytes.push(CHARS.get(byte / 16)?);
            utf8_bytes.push(CHARS.get(byte % 16)?);
        }
        Ok(String::from_utf8(utf8_bytes).unwrap())
    }
}

impl Decoder for Base16 {
    fn decode(&self, string: &str) -> EndeResult<Vec<u8>> {
        let mut bytes = Vec::new();

        for chunk in string.as_bytes().chunks(2) {
            let a = chunk[0];
            let b = *chunk
                .get(1)
                .ok_or_else(|| EndeError::new(a, "size does not match"))?;

            bytes.push(BYTES.get(a)? * 16 + BYTES.get(b)?);
        }
        Ok(bytes)
    }
}
