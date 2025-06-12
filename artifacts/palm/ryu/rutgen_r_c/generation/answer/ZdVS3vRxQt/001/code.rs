// Answer 0

#[test]
fn test_div10_basic_cases() {
    assert_eq!(div10(10), 1);
    assert_eq!(div10(20), 2);
    assert_eq!(div10(100), 10);
    assert_eq!(div10(0), 0);
}

#[test]
fn test_div10_large_numbers() {
    assert_eq!(div10(1_000), 100);
    assert_eq!(div10(1_000_000), 100_000);
    assert_eq!(div10(10_000_000_000), 1_000_000_000);
}

#[test]
fn test_div10_boundary_cases() {
    assert_eq!(div10(9), 0);
    assert_eq!(div10(19), 1);
    assert_eq!(div10(99), 9);
}

#[test]
fn test_div10_max_value() {
    assert_eq!(div10(u64::MAX), u64::MAX / 10);
}

