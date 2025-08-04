use std::ops::Range;

pub trait BitRange {
    fn bit_shl_range(&mut self, range: Range<usize>) -> bool;
    fn bit_shr_range(&mut self, range: Range<usize>) -> bool;

    fn bit_reverse_range(&mut self, range: Range<usize>) -> &mut Self;
    fn bit_not_range(&mut self, range: Range<usize>) -> &mut Self;

    fn bit_or_range(&mut self, range: Range<usize>, other: &Self) -> &mut Self;
    fn bit_and_range(&mut self, range: Range<usize>, other: &Self) -> &mut Self;
    fn bit_xor_range(&mut self, range: Range<usize>, other: &Self) -> &mut Self;

    fn bit_get_range(&self, range: Range<usize>) -> Vec<bool>;
    fn bit_set_range(&mut self, range: Range<usize>, value: bool) -> &mut Self;
}
