extern crate ende;

use ende::base64::Base64;
use ende::{Decoder, Encoder};

#[test]
fn encoding_no_padding() {
    assert_eq!(
        "YW55IGNhcm5hbCBwbGVhc3Vy".to_owned(),
        Base64.encode(b"any carnal pleasur").unwrap()
    )
}

#[test]
fn encoding_single_padding() {
    assert_eq!(
        "YW55IGNhcm5hbCBwbGVhc3VyZS4=".to_owned(),
        Base64.encode(b"any carnal pleasure.").unwrap()
    )
}

#[test]
fn encoding_double_padding() {
    assert_eq!(
        "YW55IGNhcm5hbCBwbGVhc3VyZQ==".to_owned(),
        Base64.encode(b"any carnal pleasure").unwrap()
    )
}

#[test]
fn decoding_no_padding() {
    assert_eq!(
        b"any carnal pleasur" as &[u8],
        &Base64.decode("YW55IGNhcm5hbCBwbGVhc3Vy").unwrap() as &[u8]
    )
}

#[test]
fn decoding_single_padding() {
    assert_eq!(
        b"any carnal pleasure." as &[u8],
        &Base64.decode("YW55IGNhcm5hbCBwbGVhc3VyZS4=").unwrap() as &[u8]
    )
}

#[test]
fn decoding_double_padding() {
    assert_eq!(
        b"any carnal pleasure" as &[u8],
        &Base64.decode("YW55IGNhcm5hbCBwbGVhc3VyZQ==").unwrap() as &[u8]
    )
}
