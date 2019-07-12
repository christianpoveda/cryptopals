use lazy_static::lazy_static;
use std::collections::HashMap;

const CHARS: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
lazy_static! {
    static ref BYTES: HashMap<u8, u8> = CHARS
        .iter()
        .enumerate()
        .map(|(i, &b)| (b, i as u8))
        .collect();
}
const PAD: u8 = b'=';

pub struct Encoder;

impl crate::Encoder for Encoder {
    fn encode(&self, bytes: &[u8]) -> String {
        let chars: Vec<u8> = bytes
            .chunks(3)
            .flat_map(|octs| match octs {
                [a, b, c] => vec![
                    CHARS[a.wrapping_shr(2) as usize],
                    CHARS[(a.wrapping_shl(6).wrapping_shr(2) + b.wrapping_shr(4)) as usize],
                    CHARS[(b.wrapping_shl(4).wrapping_shr(2) + c.wrapping_shr(6)) as usize],
                    CHARS[c.wrapping_shl(2).wrapping_shr(2) as usize],
                ],
                [a, b] => vec![
                    CHARS[a.wrapping_shr(2) as usize],
                    CHARS[(a.wrapping_shl(6).wrapping_shr(2) + b.wrapping_shr(4)) as usize],
                    CHARS[b.wrapping_shl(4).wrapping_shr(2) as usize],
                    PAD,
                ],
                [a] => vec![
                    CHARS[a.wrapping_shr(2) as usize],
                    CHARS[a.wrapping_shl(6).wrapping_shr(2) as usize],
                    PAD,
                    PAD
                ],
                _ => unreachable!(),
            })
            .collect();

        String::from_utf8(chars).unwrap()
    }
}

pub struct Decoder;

impl crate::Decoder for Decoder {
    fn decode(&self, string: &str) -> Vec<u8> {
        unreachable!()
    }
}
