pub mod base16;
pub mod base64;

pub trait Encoder {
    fn encode(&self, bytes: &[u8]) -> String;
}

pub trait Decoder {
    fn decode(&self, string: &str) -> Vec<u8>;
}

pub struct Converter<D: Decoder, E: Encoder> {
    decoder: D,
    encoder: E,
}

impl<D: Decoder, E: Encoder> Converter<D, E> {
    pub fn new(decoder: D, encoder: E) -> Self {
        Converter { decoder, encoder }
    }

    pub fn convert(&self, string: &str) -> String {
        self.encoder.encode(&self.decoder.decode(string))
    }
}
