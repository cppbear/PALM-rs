// Answer 0

#[test]
fn test_div100_zero() {
    let result = div100(0);
    assert_eq!(result, 0);
}

#[test]
fn test_div100_small_value() {
    let result = div100(99);
    assert_eq!(result, 0);
}

#[test]
fn test_div100_exact_hundred() {
    let result = div100(100);
    assert_eq!(result, 1);
}

#[test]
fn test_div100_large_value() {
    let result = div100(123456);
    assert_eq!(result, 1234);
}

#[test]
fn test_div100_boundary_large() {
    let result = div100(u64::MAX);
    assert_eq!(result, u64::MAX / 100);
}

