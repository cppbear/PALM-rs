// Answer 0

#[test]
fn test_u128_mulhi_small_numbers() {
    let x: u128 = 1;
    let y: u128 = 2;
    let result = u128_mulhi(x, y);
    assert_eq!(result, 0);
}

#[test]
fn test_u128_mulhi_edge_case() {
    let x: u128 = u128::MAX; // max value for u128
    let y: u128 = 1;
    let result = u128_mulhi(x, y);
    assert_eq!(result, u128::MAX >> 64);
}

#[test]
fn test_u128_mulhi_large_numbers() {
    let x: u128 = 0x0000000000000001FFFFFFFFFFFFFFFF; // x_hi = 1, x_lo = 0xFFFFFFFFFFFFFFFF
    let y: u128 = 0x0000000000000001FFFFFFFFFFFFFFFF; // y_hi = 1, y_lo = 0xFFFFFFFFFFFFFFFF
    let result = u128_mulhi(x, y);
    assert_eq!(result, 1);
}

#[test]
fn test_u128_mulhi_zero() {
    let x: u128 = 0;
    let y: u128 = 0;
    let result = u128_mulhi(x, y);
    assert_eq!(result, 0);
}

#[test]
fn test_u128_mulhi_one() {
    let x: u128 = 1;
    let y: u128 = u128::MAX;
    let result = u128_mulhi(x, y);
    assert_eq!(result, u128::MAX >> 64);
}

#[test]
fn test_u128_mulhi_max_value() {
    let x: u128 = u128::MAX;
    let y: u128 = u128::MAX;
    let result = u128_mulhi(x, y);
    assert_eq!(result, 0); 
}

