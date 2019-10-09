use std::collections::HashMap;

use analysis::freq::{bytes_freq, lang::english_freqs};
use ende::base16::Base16;
use ende::{Decoder, EndeResult};
use xor::Xor;

fn main() -> EndeResult<()> {
    let input =
        Base16.decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")?;

    let (key, _) = (u8::min_value()..=u8::max_value())
        .map(|key| (key, score(key, &english_freqs(), &bytes_freq(&input))))
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap();

    assert_eq!(
        Ok("Cooking MC\'s like a pound of bacon"),
        std::str::from_utf8(&input.xor(key))
    );

    Ok(())
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
