// Answer 0

#[test]
fn test_u128_mulhi_small_values() {
    let x: u128 = 1;
    let y: u128 = 2;
    assert_eq!(u128_mulhi(x, y), 0);
}

#[test]
fn test_u128_mulhi_large_values() {
    let x: u128 = 1 << 128 - 1; // maximum value for u128
    let y: u128 = 1 << 128 - 1;
    assert_eq!(u128_mulhi(x, y), (1 << 128)); // expected high bits from the multiplication
}

#[test]
fn test_u128_mulhi_mixed_values() {
    let x: u128 = 0x0000_0000_0000_0000_0000_0000_FFFFFFFF_FFFFFFFF; // maximum lower half
    let y: u128 = 0x0000_0000_0000_0000_0000_0000_FFFFFFFF_FFFFFFFF; // maximum lower half
    assert_eq!(u128_mulhi(x, y), 0x0000_0000_0000_0000_0000_0000_FFFFFFFF); // expected high bits
}

#[test]
fn test_u128_mulhi_zero_value() {
    let x: u128 = 0;
    let y: u128 = 0;
    assert_eq!(u128_mulhi(x, y), 0);
}

#[test]
fn test_u128_mulhi_boundary_values() {
    let x: u128 = 1;
    let y: u128 = u128::MAX; // maximum possible u128 value
    assert_eq!(u128_mulhi(x, y), 1); // expected high bits from the multiplication
}

