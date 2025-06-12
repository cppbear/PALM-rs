// Answer 0

#[test]
fn test_div5_zero() {
    let result = div5(0);
    assert_eq!(result, 0);
}

#[test]
fn test_div5_small_number() {
    let result = div5(1);
    assert_eq!(result, 0);
}

#[test]
fn test_div5_exact_multiple() {
    let result = div5(10);
    assert_eq!(result, 2);
}

#[test]
fn test_div5_large_number() {
    let result = div5(1_000_000);
    assert_eq!(result, 200_000);
}

#[test]
fn test_div5_edge_case() {
    let result = div5(4);
    assert_eq!(result, 0);
}

#[test]
fn test_div5_max_u64() {
    let result = div5(u64::MAX);
    assert_eq!(result, u64::MAX / 5);
}

