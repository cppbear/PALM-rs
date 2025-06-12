// Answer 0

#[test]
fn test_u128_mulhi_zero_case() {
    assert_eq!(u128_mulhi(0, 0), 0);
    assert_eq!(u128_mulhi(0, 1), 0);
    assert_eq!(u128_mulhi(1, 0), 0);
}

#[test]
fn test_u128_mulhi_max_values() {
    let x: u128 = u128::MAX;
    let y: u128 = u128::MAX;
    assert_eq!(u128_mulhi(x, y), (x >> 64) * (y >> 64) + ((x as u64) * (y as u64)) >> 64);
}

#[test]
fn test_u128_mulhi_simple_case() {
    let x: u128 = 1 << 64;  // 2^64
    let y: u128 = 2;        // 2
    assert_eq!(u128_mulhi(x, y), 2);
}

#[test]
fn test_u128_mulhi_large_operands() {
    let x: u128 = 1 << 63;  // 2^63
    let y: u128 = 1 << 63;  // 2^63
    assert_eq!(u128_mulhi(x, y), (x >> 64) * (y >> 64));
}

#[test]
fn test_u128_mulhi_mid_range() {
    let x: u128 = 0x00000001_00000000_00000000_00000000;  // 2^32
    let y: u128 = 0x00000000_00000001_00000000_00000000;  // 2^32
    assert_eq!(u128_mulhi(x, y), 0);
}

