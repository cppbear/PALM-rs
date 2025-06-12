// Answer 0

#[derive(Debug, PartialEq)]
struct BitMask(u64);

const BITMASK_MASK: u64 = 0xFFFFFFFFFFFFFFFF;

impl BitMask {
    pub(crate) fn invert(self) -> Self {
        BitMask(self.0 ^ BITMASK_MASK)
    }
}

#[test]
fn test_invert_all_zero_bits() {
    let bitmask = BitMask(0x0000000000000000);
    let result = bitmask.invert();
    assert_eq!(result, BitMask(0xFFFFFFFFFFFFFFFF));
}

#[test]
fn test_invert_all_one_bits() {
    let bitmask = BitMask(0xFFFFFFFFFFFFFFFF);
    let result = bitmask.invert();
    assert_eq!(result, BitMask(0x0000000000000000));
}

#[test]
fn test_invert_half_set_bits() {
    let bitmask = BitMask(0x5555555555555555); // 0101010101010101... in binary
    let result = bitmask.invert();
    assert_eq!(result, BitMask(0xAAAAAAAAAAAAAAAA)); // 1010101010101010... in binary
}

#[test]
fn test_invert_half_clear_bits() {
    let bitmask = BitMask(0xAAAAAAAAAAAAAAAA); // 1010101010101010... in binary
    let result = bitmask.invert();
    assert_eq!(result, BitMask(0x5555555555555555)); // 0101010101010101... in binary
}

#[test]
fn test_invert_random_bits() {
    let bitmask = BitMask(0x1234567890ABCDEF);
    let result = bitmask.invert();
    assert_eq!(result, BitMask(0xEDCBA98765432120));
}

