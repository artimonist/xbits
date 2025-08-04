pub struct Bits {
    data: Vec<u8>, // Storage for bits, each byte can hold 8 bits
    len: usize,    // Number of bits stored
}

///
/// ```
/// # use xbits::Bits;
/// let bits: Bits = 0b10101010_11110000_u16.into();
/// assert_eq!(bits.len(), 16);
/// assert_eq!(bits.capacity(), 16);
/// ```
impl Bits {
    pub fn new(len: usize) -> Self {
        Bits {
            data: vec![0; (len + 7) / 8],
            len,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.data.len() * 8
    }
}

macro_rules! impl_from {
    ($t:ty) => {
        impl From<$t> for Bits {
            fn from(value: $t) -> Self {
                Bits {
                    data: value.to_be_bytes().to_vec(),
                    len: std::mem::size_of::<$t>() * 8,
                }
            }
        }
    };
}

impl_from!(u8);
impl_from!(u16);
impl_from!(u32);
impl_from!(u64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_new() {
        let bits = Bits::new(17);
        assert_eq!(bits.len(), 17);
        assert_eq!(bits.capacity(), 24); // 3 bytes for 16 bits
        assert!(!bits.is_empty());
    }

    #[test]
    fn test_bits_from() {
        // u8
        let bits: Bits = 0b10101010_u8.into();
        assert_eq!(bits.len(), 8);
        assert_eq!(bits.data, [0b10101010]);
        assert_eq!(bits.capacity(), 8);

        // u64
        let bits: Bits = 0x123456789ABCDEF0_u64.into();
        assert_eq!(bits.len(), 64);
        assert_eq!(
            bits.data,
            vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]
        );
    }
}
