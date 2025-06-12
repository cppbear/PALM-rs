// Answer 0

#[test]
fn test_div10_small_value() {
    let result = div10(1);
    assert_eq!(result, 0);
}

#[test]
fn test_div10_boundary_value() {
    let result = div10(10);
    assert_eq!(result, 1);
}

#[test]
fn test_div10_large_value() {
    let result = div10(100);
    assert_eq!(result, 10);
}

#[test]
fn test_div10_zero() {
    let result = div10(0);
    assert_eq!(result, 0);
}

#[test]
fn test_div10_large_number() {
    let result = div10(1_000_000);
    assert_eq!(result, 100_000);
}

