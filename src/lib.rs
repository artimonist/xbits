//! Bit operations on `[u8]`
//!
//! # Usage
//!   `[u16]` chunks to mnemonic indices by 11 bits.  
//!   `[u8]` chunks to base64 indices by 6 bits.  
//!
//! # Examples
//! ```
//! use xbits::XBits;
//! use xbits::FromBits;
//!
//! assert_eq!(vec![0b1111_1111, 0b1100_0000].bits().all_one(), false);
//! assert_eq!(vec![0b1111_1111, 0b1100_0000].bits().trailing_zeros(), 6);
//!
//! assert_eq!(
//!     [0b1111_0000_u8].bits().iter().collect::<Vec<bool>>(),
//!     vec![true, true, true, true, false, false, false, false]
//! );
//!
//! assert_eq!(
//!     vec![0b1111_1111, 0b1111_1111].bits().chunks(6).collect::<Vec<u8>>(),
//!     vec![0b11_1111, 0b11_1111, 0b11_1100]
//! );
//!
//! assert_eq!(
//!     vec![0b1111_1111; 3].bits().chunks(11).collect::<Vec<u16>>(),
//!     vec![0b111_1111_1111, 0b111_1111_1111, 0b110_0000_0000]
//! );
//!
//! assert_eq!(
//!    vec![0b1111_1111, 0b1100_0000].bits_mut().reverse().to_ref().as_bytes(),
//!    vec![0b0000_0011, 0b1111_1111]
//! );
//!
//! assert_eq!(
//!     Vec::from_bits([true, true, true, true, false, false, false, false].iter().copied()),
//!     [0b1111_0000]
//! );
//!
//! assert_eq!(
//!     Vec::from_bits_chunk([0b11_1111_u8, 0b11_1111, 0b11_1111].into_iter(), 6),
//!     vec![0b1111_1111, 0b1111_1111, 0b1100_0000]
//! );
//!
//! assert_eq!(
//!     Vec::from_bits_chunk([0b1111_u16, 0b1111, 0b1111].into_iter(), 6),
//!     vec![0b001111_00, 0b1111_0011, 0b1100_0000]
//! );
//! ```

pub mod core;
mod xbits;

pub use core::FromBits;
pub use xbits::{BitsMut, BitsRef, XBits};

/// Assert overflow of parameter
/// # Parameters
/// - $n: parameter
/// - $min: minimum value
/// - $max: maximum value
/// - $name: function name
macro_rules! assert_overflow {
    ($n: ident, $min: expr, $max: expr, $name: literal) => {
        let (name, param, n, min, max) = ($name, stringify!($n), $n, $min, $max);
        assert!(
            $min as usize <= $n as usize && $n as usize <= $max as usize,
            "[xbits] {name} parameter `{param}` overflow: `{n}` not in `{min}..={max}`",
        );
    };
}
pub(crate) use assert_overflow;
