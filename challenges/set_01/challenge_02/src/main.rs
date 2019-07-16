use ende::{Decoder, EndeResult};
use ende::base16::Base16;


fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
}


fn main() -> EndeResult<()> {
    let input_a = Base16.decode("1c0111001f010100061a024b53535009181c")?;
    let input_b = Base16.decode("686974207468652062756c6c277320657965")?;
    let expect = Base16.decode("746865206b696420646f6e277420706c6179")?;
    let result = xor(&input_a, &input_b);
    assert_eq!(result, expect);
    Ok(())
}
