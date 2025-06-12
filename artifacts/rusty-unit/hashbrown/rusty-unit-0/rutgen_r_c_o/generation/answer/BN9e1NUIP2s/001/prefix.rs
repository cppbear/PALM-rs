// Answer 0

#[test]
fn test_trailing_zeros_case_1() {
    let bitmask = BitMask(0b0000_0001);
    let result = bitmask.trailing_zeros();
}

#[test]
fn test_trailing_zeros_case_2() {
    let bitmask = BitMask(0b0000_0010);
    let result = bitmask.trailing_zeros();
}

#[test]
fn test_trailing_zeros_case_3() {
    let bitmask = BitMask(0b0000_1110);
    let result = bitmask.trailing_zeros();
}

#[test]
fn test_trailing_zeros_case_4() {
    let bitmask = BitMask(0b1111_1111);
    let result = bitmask.trailing_zeros();
}

#[test]
fn test_trailing_zeros_case_5() {
    let bitmask = BitMask(0);
    let result = bitmask.trailing_zeros();
}

#[test]
fn test_trailing_zeros_edge_case() {
    let bitmask = BitMask(BITMASK_MASK >> BITMASK_STRIDE);
    let result = bitmask.trailing_zeros();
}

