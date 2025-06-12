// Answer 0

#[test]
fn test_new_with_minimum_values() {
    let state: u128 = 0x00000000000000000000000000000000;
    let stream: u128 = 0x00000000000000000000000000000000;
    let rng = Lcg128Xsl64::new(state, stream);
}

#[test]
fn test_new_with_maximum_state_minimum_stream() {
    let state: u128 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
    let stream: u128 = 0x00000000000000000000000000000000;
    let rng = Lcg128Xsl64::new(state, stream);
}

#[test]
fn test_new_with_middle_range_values() {
    let state: u128 = 0x7FFFFFFFFFFFFFFF7FFFFFFFFFFFFFFF;
    let stream: u128 = 0x3FFFFFFFFFFFFFFF;
    let rng = Lcg128Xsl64::new(state, stream);
}

#[test]
fn test_new_with_odd_stream() {
    let state: u128 = 0x5A5A5A5A5A5A5A5A5A5A5A5A5A5A5A5A;
    let stream: u128 = 0x1FFFFFFFFFFFFFFF;
    let rng = Lcg128Xsl64::new(state, stream);
}

#[test]
fn test_new_with_bound_values() {
    let state: u128 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
    let stream: u128 = 0x7FFFFFFFFFFFFFFF;
    let rng = Lcg128Xsl64::new(state, stream);
}

#[test]
fn test_new_with_small_increment_stream() {
    let state: u128 = 0x1A1A1A1A1A1A1A1A1A1A1A1A1A1A1A1A;
    let stream: u128 = 0x0000000000000001;
    let rng = Lcg128Xsl64::new(state, stream);
}

