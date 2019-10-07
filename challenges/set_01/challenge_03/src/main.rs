use std::collections::HashMap;

use ende::base16::Base16;
use ende::{Decoder, EndeResult};
use xor::Xor;

fn main() -> EndeResult<()> {
    let input =
        Base16.decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")?;

    let freqs = count_bytes(&input);

    let (key, _) = (u8::min_value()..=u8::max_value())
        .map(|key| (key, score(key, &english_freqs(), &freqs)))
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap();

    assert_eq!(
        Ok("Cooking MC\'s like a pound of bacon"),
        std::str::from_utf8(&input.xor(key))
    );

    Ok(())
}

fn count_bytes(data: &[u8]) -> HashMap<u8, f64> {
    let mut map = HashMap::new();
    let total = data.len() as f64;
    for &byte in data {
        *map.entry(byte).or_insert(0) += 1;
    }

    map.into_iter()
        .map(|(k, v)| (k, v as f64 / total))
        .collect()
}

fn score(key: u8, a: &HashMap<u8, f64>, b: &HashMap<u8, f64>) -> f64 {
    let mut score = 0.0;
    for (k_a, v_a) in a {
        let k_b = (k_a ^ key).to_ascii_lowercase();
        let v_b = b.get(&k_b).unwrap_or(&0.0);
        score += (v_a - v_b).powi(2);
    }
    score
}

fn english_freqs() -> HashMap<u8, f64> {
    [
        (b'i', 0.053227986277159786),
        (b't', 0.07246075475621166),
        (b' ', 0.1604636656617112),
        (b's', 0.04589874207298056),
        (b'a', 0.06154485913296601),
        (b'm', 0.020480299407422808),
        (b'e', 0.09850296288595488),
        (b'l', 0.033423432789271235),
        (b'n', 0.05956960182971203),
        (b'c', 0.02032435804137644),
        (b'h', 0.04387150431437779),
        (b'o', 0.0644037841771494),
        (b'y', 0.016581765256263644),
        (b'b', 0.015022351595799979),
        (b'j', 0.0009876286516269882),
        (b'w', 0.015334234327892712),
        (b'k', 0.004938143258134941),
        (b'r', 0.05192847489344007),
        (b'u', 0.023391204906954986),
        (b'g', 0.015646117059985445),
        (b'v', 0.008940638319991683),
        (b'd', 0.030720449111134213),
        (b'f', 0.02084416259486433),
        (b'x', 0.0016113941158124546),
        (b'p', 0.015750077970683025),
        (b'q', 0.0010396091069757771),
        (b'z', 0.00010396091069757771),
    ]
    .into_iter()
    .cloned()
    .collect::<HashMap<u8, f64>>()
}
