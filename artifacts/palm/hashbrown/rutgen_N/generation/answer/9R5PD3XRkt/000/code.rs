// Answer 0

#[derive(Debug)]
struct BitMask(u32);

const BITMASK_STRIDE: usize = 32;

impl BitMask {
    pub(crate) fn leading_zeros(self) -> usize {
        self.0.leading_zeros() as usize / BITMASK_STRIDE
    }
}

#[test]
fn test_leading_zeros_all_ones() {
    let bitmask = BitMask(0xFFFFFFFF);
    assert_eq!(bitmask.leading_zeros(), 0);
}

#[test]
fn test_leading_zeros_some_zeros() {
    let bitmask = BitMask(0x0F00_0000);
    assert_eq!(bitmask.leading_zeros(), 1);
}

#[test]
fn test_leading_zeros_all_zeros() {
    let bitmask = BitMask(0x0000_0000);
    assert_eq!(bitmask.leading_zeros(), 1);
}

#[test]
fn test_leading_zeros_boundary_case() {
    let bitmask = BitMask(0x8000_0000);
    assert_eq!(bitmask.leading_zeros(), 0);
}

