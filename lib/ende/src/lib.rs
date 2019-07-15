use std::collections::HashMap;
use std::fmt;

pub mod base16;
pub mod base64;

pub trait Encoder {
    fn encode(&self, bytes: &[u8]) -> EndeResult<String>;
}

pub trait Decoder {
    fn decode(&self, string: &str) -> EndeResult<Vec<u8>>;
}

pub type EndeResult<T> = Result<T, EndeError>;

#[derive(Debug)]
pub struct EndeError {
    byte: u8,
    reason: &'static str,
}

impl EndeError {
    pub fn new(byte: u8, reason: &'static str) -> Self {
        EndeError { byte, reason }
    }
}

impl fmt::Display for EndeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error at byte {:?} with reason: {}",
            self.byte, self.reason
        )
    }
}

pub struct Converter<D: Decoder, E: Encoder> {
    decoder: D,
    encoder: E,
}

impl<D: Decoder, E: Encoder> Converter<D, E> {
    pub fn new(decoder: D, encoder: E) -> Self {
        Converter { decoder, encoder }
    }

    pub fn convert(&self, string: &str) -> EndeResult<String> {
        self.encoder.encode(&self.decoder.decode(string)?)
    }
}

enum Table<'a> {
    Slice(&'a [u8]),
    HashMap(HashMap<u8, u8>),
}

impl<'a> Table<'a> {
    fn get(&self, byte: u8) -> EndeResult<u8> {
        match self {
            Table::Slice(slice) => slice.get(byte as usize),
            Table::HashMap(hash_map) => hash_map.get(&byte),
        }
        .cloned()
        .ok_or_else(|| EndeError::new(byte, "byte out of range"))
    }

    fn transpose(&self) -> Self {
        match self {
            Table::Slice(slice) => Table::HashMap(
                slice
                    .iter()
                    .enumerate()
                    .map(|(i, &b)| (b, i as u8))
                    .collect(),
            ),
            Table::HashMap(hash_map) => {
                Table::HashMap(hash_map.iter().map(|(&k, &v)| (v, k)).collect())
            }
        }
    }
}
