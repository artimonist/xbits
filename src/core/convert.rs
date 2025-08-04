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

impl_to_bits!(i8);
impl_to_bits!(i16);
impl_to_bits!(i32);
impl_to_bits!(i64);
impl_to_bits!(i128);
impl_to_bits!(isize);
