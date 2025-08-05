pub trait ToBits {
    fn to_bits(self) -> Vec<u8>;
}

macro_rules! impl_to_bits {
    ($t:ty) => {
        impl ToBits for $t {
            fn to_bits(self) -> Vec<u8> {
                self.to_be_bytes().to_vec()
            }
        }
    };
}

impl_to_bits!(u8);
impl_to_bits!(u16);
impl_to_bits!(u32);
impl_to_bits!(u64);
impl_to_bits!(u128);
impl_to_bits!(usize);

impl<T> ToBits for Vec<T>
where
    T: ToBits + Copy,
{
    fn to_bits(self) -> Vec<u8> {
        self.iter().flat_map(|&x| x.to_bits()).collect::<Vec<_>>()
    }
}

impl<T> ToBits for &[T]
where
    T: ToBits + Copy,
{
    fn to_bits(self) -> Vec<u8> {
        self.iter().flat_map(|&x| x.to_bits()).collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::ToBits;
    #[test]
    fn test_to_bits() {
        assert_eq!(42_u8.to_bits(), vec![42]);
        assert_eq!(42_u16.to_bits(), vec![0, 42]);
        assert_eq!(42_u32.to_bits(), vec![0, 0, 0, 42]);
        assert_eq!(42_u64.to_bits(), vec![0, 0, 0, 0, 0, 0, 0, 42]);
        assert_eq!(42_u128.to_bits(), [&[0; 15][..], &[42]].concat());
    }

    #[test]
    fn test_to_bits_iter() {
        assert_eq!(vec![1u8, 2, 3].to_bits(), vec![1, 2, 3]);
        assert_eq!([1u16, 2, 3].to_bits(), vec![0, 1, 0, 2, 0, 3]);
        let slice = &[1u32, 2, 3][..];
        assert_eq!(slice.to_bits(), vec![0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]);
    }
}
