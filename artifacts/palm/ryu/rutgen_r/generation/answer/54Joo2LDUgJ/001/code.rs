// Answer 0

#[test]
fn test_div100_basic() {
    assert_eq!(div100(200), 2);
    assert_eq!(div100(100), 1);
    assert_eq!(div100(0), 0);
}

#[test]
fn test_div100_large() {
    assert_eq!(div100(10000), 100);
    assert_eq!(div100(123456), 1234);
}

#[test]
fn test_div100_boundary() {
    assert_eq!(div100(99), 0);
    assert_eq!(div100(10000), 100);
    assert_eq!(div100(u64::MAX), 18446744073709551615);
}

