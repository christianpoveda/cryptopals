use lazy_static::lazy_static;

use crate::{Decoder, Encoder, EndeError, EndeResult, Table};

const CHARS: Table =
    Table::Slice(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
lazy_static! {
    static ref BYTES: Table<'static> = CHARS.transpose();
}

const PAD: u8 = b'=';

pub struct Base64;

impl Encoder for Base64 {
    fn encode(&self, bytes: &[u8]) -> EndeResult<String> {
        let chars: EndeResult<Vec<u8>> = bytes
            .chunks(3)
            .flat_map(|octs| match octs {
                [a, b, c] => vec![
                    CHARS.get(a.wrapping_shr(2)),
                    CHARS.get(a.wrapping_shl(6).wrapping_shr(2) + b.wrapping_shr(4)),
                    CHARS.get(b.wrapping_shl(4).wrapping_shr(2) + c.wrapping_shr(6)),
                    CHARS.get(c.wrapping_shl(2).wrapping_shr(2)),
                ],
                [a, b] => vec![
                    CHARS.get(a.wrapping_shr(2)),
                    CHARS.get(a.wrapping_shl(6).wrapping_shr(2) + b.wrapping_shr(4)),
                    CHARS.get(b.wrapping_shl(4).wrapping_shr(2)),
                    Ok(PAD),
                ],
                [a] => vec![
                    CHARS.get(a.wrapping_shr(2)),
                    CHARS.get(a.wrapping_shl(6).wrapping_shr(2)),
                    Ok(PAD),
                    Ok(PAD),
                ],
                _ => unreachable!(),
            })
            .collect();

        Ok(String::from_utf8(chars?).unwrap())
    }
}

impl Decoder for Base64 {
    fn decode(&self, string: &str) -> EndeResult<Vec<u8>> {
        let mut bytes = Vec::new();
        for chunk in string.as_bytes().chunks(4) {
            match chunk {
                [a, b, PAD, PAD] => {
                    let a = BYTES.get(*a)?;
                    let b = BYTES.get(*b)?;

                    bytes.push(a.wrapping_shl(2) + b.wrapping_shr(4));
                }
                [a, b, c, PAD] => {
                    let a = BYTES.get(*a)?;
                    let b = BYTES.get(*b)?;
                    let c = BYTES.get(*c)?;
                    bytes.push(a.wrapping_shl(2) + b.wrapping_shr(4));
                    bytes.push(b.wrapping_shl(4) + c.wrapping_shr(2));
                }
                [a, b, c, d] => {
                    let a = BYTES.get(*a)?;
                    let b = BYTES.get(*b)?;
                    let c = BYTES.get(*c)?;
                    let d = BYTES.get(*d)?;
                    bytes.push(a.wrapping_shl(2) + b.wrapping_shr(4));
                    bytes.push(b.wrapping_shl(4) + c.wrapping_shr(2));
                    bytes.push(c.wrapping_shl(6) + d);
                }
                _ => return Err(EndeError::new(chunk[0], "size does not match")),
            }
        }
        Ok(bytes)
    }
}
