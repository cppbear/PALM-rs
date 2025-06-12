// Answer 0

#[derive(Debug)]
struct BitMask(u32);

const BITMASK_STRIDE: usize = 1;

impl BitMask {
    pub(crate) fn leading_zeros(self) -> usize {
        self.0.leading_zeros() as usize / BITMASK_STRIDE
    }
}

#[test]
fn test_leading_zeros_all_ones() {
    let mask = BitMask(0b11111111_11111111_11111111_11111111);
    assert_eq!(mask.leading_zeros(), 0);
}

#[test]
fn test_leading_zeros_some_zeros() {
    let mask = BitMask(0b00000001_11111111_11111111_11111111);
    assert_eq!(mask.leading_zeros(), 1);
}

#[test]
fn test_leading_zeros_half_zeros() {
    let mask = BitMask(0b00000000_00000000_11111111_11111111);
    assert_eq!(mask.leading_zeros(), 16);
}

#[test]
fn test_leading_zeros_all_zeros() {
    let mask = BitMask(0b00000000_00000000_00000000_00000000);
    assert_eq!(mask.leading_zeros(), 32);
}

#[test]
fn test_leading_zeros_single_zero() {
    let mask = BitMask(0b00000000_00000000_00000000_00000001);
    assert_eq!(mask.leading_zeros(), 31);
}

