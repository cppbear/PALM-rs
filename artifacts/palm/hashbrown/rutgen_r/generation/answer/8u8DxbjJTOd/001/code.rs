// Answer 0

#[derive(Debug, PartialEq)]
struct BitMask(u32);

impl BitMask {
    fn remove_lowest_bit(self) -> Self {
        BitMask(self.0 & (self.0 - 1))
    }
}

#[test]
fn test_remove_lowest_bit_non_zero() {
    let bitmask = BitMask(0b0110);
    let result = bitmask.remove_lowest_bit();
    assert_eq!(result, BitMask(0b0100));
}

#[test]
fn test_remove_lowest_bit_single_bit() {
    let bitmask = BitMask(0b0001);
    let result = bitmask.remove_lowest_bit();
    assert_eq!(result, BitMask(0b0000));
}

#[test]
fn test_remove_lowest_bit_zero() {
    let bitmask = BitMask(0b0000);
    let result = bitmask.remove_lowest_bit();
    assert_eq!(result, BitMask(0b0000));
}

#[test]
fn test_remove_lowest_bit_multiple_bits() {
    let bitmask = BitMask(0b1111);
    let result = bitmask.remove_lowest_bit();
    assert_eq!(result, BitMask(0b1110));
}

#[test]
fn test_remove_lowest_bit_large_value() {
    let bitmask = BitMask(0xFFFFFFFF);
    let result = bitmask.remove_lowest_bit();
    assert_eq!(result, BitMask(0xFFFFFFFE));
}

