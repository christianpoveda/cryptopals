extern crate ende;

use ende::base64;
use ende::{Decoder, Encoder};

#[test]
fn encoding_no_padding() {
    assert_eq!(
        "YW55IGNhcm5hbCBwbGVhc3Vy".to_owned(),
        base64::Encoder.encode(b"any carnal pleasur")
    )
}

#[test]
fn encoding_single_padding() {
    assert_eq!(
        "YW55IGNhcm5hbCBwbGVhc3VyZS4=".to_owned(),
        base64::Encoder.encode(b"any carnal pleasure.")
    )
}

#[test]
fn encoding_double_padding() {
    assert_eq!(
        "YW55IGNhcm5hbCBwbGVhc3VyZQ==".to_owned(),
        base64::Encoder.encode(b"any carnal pleasure")
    )
}
