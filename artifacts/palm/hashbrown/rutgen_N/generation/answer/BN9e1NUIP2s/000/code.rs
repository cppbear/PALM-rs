// Answer 0

#[test]
fn test_trailing_zeros_no_trailing_zeros() {
    struct BitMask(u64);
    
    let mask = BitMask(0b1111); // 0b1111 has no trailing zeros
    assert_eq!(mask.trailing_zeros(), 0);
}

#[test]
fn test_trailing_zeros_one_trailing_zero() {
    struct BitMask(u64);
    
    let mask = BitMask(0b0110); // 0b0110 has one trailing zero
    assert_eq!(mask.trailing_zeros(), 1);
}

#[test]
fn test_trailing_zeros_two_trailing_zeros() {
    struct BitMask(u64);
    
    let mask = BitMask(0b0010); // 0b0010 has two trailing zeros
    assert_eq!(mask.trailing_zeros(), 2);
}

#[test]
fn test_trailing_zeros_all_ones() {
    struct BitMask(u64);
    
    let mask = BitMask(0b1111111111111111); // All ones has no trailing zeros
    assert_eq!(mask.trailing_zeros(), 0);
}

#[test]
fn test_trailing_zeros_zero() {
    struct BitMask(u64);
    
    let mask = BitMask(0b0000); // Zero has a number of trailing zeros
    assert_eq!(mask.trailing_zeros(), 4); // 4 trailing zeros in a 4 bit mask
}

#[test]
fn test_trailing_zeros_large_number() {
    struct BitMask(u64);
    
    let mask = BitMask(0b1000000000000000); // has 15 trailing zeros
    assert_eq!(mask.trailing_zeros(), 15);
}

