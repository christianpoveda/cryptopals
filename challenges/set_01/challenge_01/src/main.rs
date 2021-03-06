use ende::{base16::Base16, base64::Base64, Converter};

fn main() {
    let conv = Converter::new(Base16, Base64);
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let result = conv.convert(input).unwrap();
    let expect = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_owned();
    assert_eq!(result, expect);
}
