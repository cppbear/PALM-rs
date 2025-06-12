// Answer 0

#[test]
fn test_div100_zero() {
    let result = div100(0);
    assert_eq!(result, 0);
}

#[test]
fn test_div100_small_value() {
    let result = div100(50);
    assert_eq!(result, 0);
}

#[test]
fn test_div100_exact_division() {
    let result = div100(100);
    assert_eq!(result, 1);
}

#[test]
fn test_div100_large_value() {
    let result = div100(10000);
    assert_eq!(result, 100);
}

#[test]
fn test_div100_large_non_exact_value() {
    let result = div100(9999);
    assert_eq!(result, 99);
}

