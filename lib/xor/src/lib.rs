pub trait Xor<Rhs = Self> {
    type Output;

    fn xor(&self, rhs: Rhs) -> Self::Output;
}

impl Xor for &[u8] {
    type Output = Vec<u8>;

    fn xor(&self, other: &[u8]) -> Self::Output {
        self.iter().zip(other.iter()).map(|(a, b)| a ^ b).collect()
    }
}

impl Xor<&[u8]> for Vec<u8> {
    type Output = Vec<u8>;

    #[inline(always)]
    fn xor(&self, other: &[u8]) -> Self::Output {
        (&self as &[u8]).xor(other)
    }
}
