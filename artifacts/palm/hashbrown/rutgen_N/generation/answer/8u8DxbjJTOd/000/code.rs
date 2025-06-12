// Answer 0

#[derive(Debug, PartialEq)]
struct BitMask(u64);

impl BitMask {
    fn remove_lowest_bit(self) -> Self {
        BitMask(self.0 & (self.0 - 1))
    }
}

#[test]
fn test_remove_lowest_bit() {
    let mask = BitMask(0b1010);
    let result = mask.remove_lowest_bit();
    assert_eq!(result, BitMask(0b1000));
}

#[test]
fn test_remove_lowest_bit_all_bits_set() {
    let mask = BitMask(0b1111);
    let result = mask.remove_lowest_bit();
    assert_eq!(result, BitMask(0b1110));
}

#[test]
fn test_remove_lowest_bit_single_bit() {
    let mask = BitMask(0b0001);
    let result = mask.remove_lowest_bit();
    assert_eq!(result, BitMask(0b0000));
}

#[test]
fn test_remove_lowest_bit_zero() {
    let mask = BitMask(0b0000);
    let result = mask.remove_lowest_bit();
    assert_eq!(result, BitMask(0b0000));
}

