extern crate ende;

use ende::base16::*;
use ende::{Encoder, Decoder};

const DATA: [(u8, char); 16] = [
    (0, '0'),
    (1, '1'),
    (2, '2'),
    (3, '3'),
    (4, '4'),
    (5, '5'),
    (6, '6'),
    (7, '7'),
    (8, '8'),
    (9, '9'),
    (10, 'a'),
    (11, 'b'),
    (12, 'c'),
    (13, 'd'),
    (14, 'e'),
    (15, 'f'),
];

#[test]
fn single_char_encoding() {
    for &(byte, chr) in &DATA {
        assert_eq!(Base16Encoder.encode(&[byte]).chars().next().unwrap(), chr);
    }
}

#[test]
fn single_char_decoding() {
    for &(byte, chr) in &DATA {
        assert_eq!(Base16Decoder.decode(&chr.to_string())[0], byte);
    }
}
