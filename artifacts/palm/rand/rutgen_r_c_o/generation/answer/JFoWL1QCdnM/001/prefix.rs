// Answer 0

#[test]
fn test_new_minimum_values() {
    let state: u64 = 0x0;
    let stream: u64 = 0x0;
    let _rng = Lcg64Xsh32::new(state, stream);
}

#[test]
fn test_new_maximum_values() {
    let state: u64 = 0xffffffffffffffff;
    let stream: u64 = 0x7fffffffffffffff;
    let _rng = Lcg64Xsh32::new(state, stream);
}

#[test]
fn test_new_middle_values() {
    let state: u64 = 0x7fffffffffffffff;
    let stream: u64 = 0x3fffffffffffffff;
    let _rng = Lcg64Xsh32::new(state, stream);
}

#[test]
fn test_new_edge_increment() {
    let state: u64 = 0xcafef00dd15ea5e5;
    let stream: u64 = 0x1; // minimum odd
    let _rng = Lcg64Xsh32::new(state, stream);
}

#[test]
fn test_new_random_valid_values() {
    let state: u64 = 0x123456789abcdef0;
    let stream: u64 = 0x23456789abcdef0;
    let _rng = Lcg64Xsh32::new(state, stream);
}

#[test]
fn test_new_with_highest_bit_discarded() {
    let state: u64 = 0x7fffffffffffffff;
    let stream: u64 = 0x8000000000000000; // highest bit discarded, should evaluate appropriately
    let _rng = Lcg64Xsh32::new(state, stream);
}

