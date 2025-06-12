// Answer 0

#[test]
fn test_invert_all_bits_zero() {
    let bitmask = BitMask(0);
    let _ = bitmask.invert();
}

#[test]
fn test_invert_half_bits_set() {
    let bitmask = BitMask(BITMASK_MASK >> 1);
    let _ = bitmask.invert();
}

#[test]
fn test_invert_all_bits_set() {
    let bitmask = BitMask(BITMASK_MASK);
    let _ = bitmask.invert();
}

#[test]
fn test_invert_lowest_bit_set() {
    let bitmask = BitMask(1);
    let _ = bitmask.invert();
}

#[test]
fn test_invert_highest_bit_set() {
    let bitmask = BitMask(BITMASK_MASK);
    let _ = bitmask.invert();
}

#[test]
fn test_invert_random_bits() {
    let bitmask = BitMask(42);
    let _ = bitmask.invert();
}

