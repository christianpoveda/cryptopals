extern crate ende;

use ende::base16::Base16;
use ende::{Decoder, Encoder};

const DATA: [(u8, &'static str); 15] = [
    (0x10, "10"),
    (0x21, "21"),
    (0x32, "32"),
    (0x43, "43"),
    (0x54, "54"),
    (0x65, "65"),
    (0x76, "76"),
    (0x87, "87"),
    (0x98, "98"),
    (0xa9, "a9"),
    (0xba, "ba"),
    (0xcb, "cb"),
    (0xdc, "dc"),
    (0xed, "ed"),
    (0xfe, "fe")
];

#[test]
fn single_byte_encoding() {
    for &(byte, string) in &DATA {
        assert_eq!(Base16.encode(&[byte]).unwrap(), string);
    }
}

#[test]
fn single_byte_decoding() {
    for &(byte, string) in &DATA {
        assert_eq!(Base16.decode(&string).unwrap(), vec![byte]);
    }
}
