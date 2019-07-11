pub mod base16;

pub trait Encoder {
    fn encode(&self, bytes: &[u8]) -> String;
}

pub trait Decoder {
    fn decode(&self, string: &str) -> Vec<u8>;
}

pub struct Converter<E: Encoder, D: Decoder> {
    encoder: E,
    decoder: D,
}

impl<E: Encoder, D: Decoder> Converter<E, D> {
    pub fn new(encoder: E, decoder: D) -> Self {
        Converter { encoder, decoder }
    }

    pub fn convert(&self, string: &str) -> String {
        self.encoder.encode(&self.decoder.decode(string))
    }
}
