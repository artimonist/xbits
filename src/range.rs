use std::ops::Range;

pub trait BitsRange {
    fn bits_range(&self, range: Range<usize>) -> RangeRef;
}

pub trait BitsRangeMut {
    fn bits_range_mut(&mut self, range: Range<usize>) -> RangeMut;
}

pub struct RangeRef<'a> {
    data: &'a [u8],
    range: Range<usize>,
}

pub struct RangeMut<'a> {
    data: &'a mut [u8],
    range: Range<usize>,
}
