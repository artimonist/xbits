/**
 * `AsBits` and `AsBitsMut` trait provides a way to work with bit-level operations on byte arrays.
 * It allows you to get a reference to the bits in a byte array and perform operations
 * such as checking if all bits are one or zero, and iterating over the bits.
 */
use super::core::{BitIterator, Bitwise};

pub trait AsBits {
    fn as_bits(&self) -> BitsRef;
}

pub trait AsBitsMut {
    fn as_bits_mut(&mut self) -> BitsMut;
}

impl AsBits for [u8] {
    #[inline(always)]
    fn as_bits(&self) -> BitsRef {
        BitsRef(self)
    }
}

impl AsBitsMut for [u8] {
    #[inline(always)]
    fn as_bits_mut(&mut self) -> BitsMut {
        BitsMut(self)
    }
}

impl AsBits for Vec<u8> {
    #[inline(always)]
    fn as_bits(&self) -> BitsRef {
        BitsRef(self)
    }
}

impl AsBitsMut for Vec<u8> {
    #[inline(always)]
    fn as_bits_mut(&mut self) -> BitsMut {
        BitsMut(self)
    }
}

/// A wrapper to a byte array that allows for bit-level operations.
/// It provides easy to use methods for checking bit states, iterating over bits,
/// and performing bitwise operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitsRef<'a>(pub &'a [u8]);

impl BitsRef<'_> {
    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        self.0
    }

    /// Returns true if the byte array is empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the number of bits in the byte array.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.len() * 8
    }

    #[inline(always)]
    pub fn all_one(&self) -> bool {
        self.0.bit_all_one()
    }

    #[inline(always)]
    pub fn all_zero(&self) -> bool {
        self.0.bit_all_zero()
    }

    #[inline(always)]
    pub fn leading_zeros(&self) -> usize {
        self.0.bit_leading_zeros()
    }

    #[inline(always)]
    pub fn trailing_zeros(&self) -> usize {
        self.0.bit_trailing_zeros()
    }

    #[inline(always)]
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = bool> + '_ {
        self.0.bit_iter()
    }

    #[inline(always)]
    pub fn chunks<T>(&self, n: usize) -> impl std::iter::Iterator<Item = T> + '_
    where
        T: TryFrom<u64> + Default + 'static,
    {
        self.0.bit_chunks(n)
    }
}

impl std::ops::Index<usize> for BitsRef<'_> {
    type Output = bool;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        if self.0.bit_get(index) { &true } else { &false }
    }
}

impl std::fmt::Display for BitsRef<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.bit_fmt())
    }
}

/// A mutable wrapper to a byte array that allows for bit-level operations.
/// It provides methods for shifting, bitwise operations, and reversing bits.
/// It can be used to modify the underlying byte array directly.
#[derive(Debug, PartialEq, Eq)]
pub struct BitsMut<'a>(pub &'a mut [u8]);

impl BitsMut<'_> {
    #[inline(always)]
    pub fn to_ref(&self) -> BitsRef {
        BitsRef(self.0)
    }

    #[inline(always)]
    #[allow(clippy::should_implement_trait)]
    pub fn shl(self, rhs: usize) -> Self {
        self.0.bit_shl(rhs);
        self
    }

    #[inline(always)]
    #[allow(clippy::should_implement_trait)]
    pub fn shr(self, n: usize) -> Self {
        self.0.bit_shr(n);
        self
    }

    #[inline(always)]
    pub fn or(self, other: impl AsBits) -> Self {
        self.0.bit_be_or(other.as_bits().0);
        self
    }

    #[inline(always)]
    pub fn and(self, other: impl AsBits) -> Self {
        self.0.bit_be_and(other.as_bits().0);
        self
    }

    #[inline(always)]
    pub fn xor(self, other: impl AsBits) -> Self {
        self.0.bit_be_xor(other.as_bits().0);
        self
    }

    #[inline(always)]
    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> Self {
        self.0.bit_not();
        self
    }

    #[inline(always)]
    pub fn reverse(self) -> Self {
        self.0.bit_reverse();
        self
    }

    #[inline(always)]
    pub fn fill(self, value: bool) -> Self {
        self.0.bit_fill(value);
        self
    }

    #[inline(always)]
    pub fn set(self, index: usize, value: bool) -> Self {
        self.0.bit_set(index, value);
        self
    }
}

impl std::fmt::Display for BitsMut<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.bit_fmt())
    }
}

#[cfg(test)]
mod tests {
    use super::{AsBits, AsBitsMut};
    use crate::ToBits;

    #[test]
    fn test_xbits() {
        let mut buf = [0b00000001_u8, 0b00000010, 0b00000100];
        assert_eq!(buf.as_bits().to_string(), "00000001 00000010 00000100");

        // xor
        assert_eq!(
            buf.clone()
                .as_bits_mut()
                .xor(5555_u16.to_bits())
                .xor(5555_u64.to_bits())
                .0,
            buf
        );

        // index
        for i in 0..buf.len() {
            assert_eq!(buf.as_bits()[i], buf.as_bits().iter().nth(i).unwrap());
        }

        // as_ref
        let _ = buf
            .as_bits_mut()
            .and(88_u16.to_bits())
            .not()
            .set(5, true)
            .or(0xff_u32.to_bits())
            .reverse()
            .shl(3)
            .shr(5)
            .to_ref()
            .leading_zeros();

        // reverse
        let mut vs = [0b1111_1111, 0b1100_0000];
        vs.as_bits_mut().reverse();
        assert_eq!(vs, [0b0000_0011, 0b1111_1111]);
    }

    #[test]
    fn test_bitwise() {
        let mut buf = [0b00000001_u8, 0b00000010, 0b00000100];
        buf.as_bits_mut()
            .shl(1)
            .or(0b11111111_u8.to_bits())
            .xor(0b11110000_u8.to_bits());
        assert_eq!(buf, [0b00000010_u8, 0b00000100, 0b00001111]);
        assert_eq!(buf.as_bits()[6], true);
        assert_eq!(buf.as_bits()[7], false);
        buf.as_bits_mut().set(11, true);
        assert_eq!(buf.as_bits()[11], true);

        assert_eq!(buf.as_bits_mut().fill(false).to_ref().all_zero(), true);
        assert_eq!(buf.as_bits_mut().fill(true).to_ref().all_one(), true);
    }
}
