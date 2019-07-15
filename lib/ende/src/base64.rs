use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::{Decoder, Encoder};

const CHARS: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
lazy_static! {
    static ref BYTES: HashMap<u8, u8> = CHARS
        .iter()
        .enumerate()
        .map(|(i, &b)| (b, i as u8))
        .collect();
}
const PAD: u8 = b'=';

pub struct Base64;

impl Encoder for Base64 {
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
                    PAD,
                ],
                _ => unreachable!(),
            })
            .collect();

        String::from_utf8(chars).unwrap()
    }
}

impl Decoder for Base64 {
    fn decode(&self, string: &str) -> Vec<u8> {
        string
            .as_bytes()
            .chunks(4)
            .flat_map(|sexts| match sexts {
                [a, b, PAD, PAD] => {
                    let a = BYTES[a];
                    let b = BYTES[b];
                    vec![a.wrapping_shl(2) + b.wrapping_shr(4)]
                }
                [a, b, c, PAD] => {
                    let a = BYTES[a];
                    let b = BYTES[b];
                    let c = BYTES[c];
                    vec![
                        a.wrapping_shl(2) + b.wrapping_shr(4),
                        b.wrapping_shl(4) + c.wrapping_shr(2),
                    ]
                }
                [a, b, c, d] => {
                    let a = BYTES[a];
                    let b = BYTES[b];
                    let c = BYTES[c];
                    let d = BYTES[d];
                    vec![
                        a.wrapping_shl(2) + b.wrapping_shr(4),
                        b.wrapping_shl(4) + c.wrapping_shr(2),
                        c.wrapping_shl(6) + d,
                    ]
                }
                _ => unreachable!(),
            })
            .collect()
    }
}
