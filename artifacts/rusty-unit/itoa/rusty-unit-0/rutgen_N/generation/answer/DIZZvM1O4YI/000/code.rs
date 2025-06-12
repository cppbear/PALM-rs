// Answer 0

#[test]
fn test_u128_mulhi_small_numbers() {
    let result = u128_mulhi(2, 3);
    assert_eq!(result, 0);
}

#[test]
fn test_u128_mulhi_large_numbers() {
    let result = u128_mulhi((1u128 << 64) - 1, (1u128 << 64) - 1);
    assert_eq!(result, (1u128 << 64) - 1);
}

#[test]
fn test_u128_mulhi_edge_cases() {
    let result1 = u128_mulhi(0, 0);
    assert_eq!(result1, 0);

    let result2 = u128_mulhi(u128::MAX, 1);
    assert_eq!(result2, u128::MAX >> 64);

    let result3 = u128_mulhi(u128::MAX, u128::MAX);
    assert_eq!(result3, u128::MAX);
}

