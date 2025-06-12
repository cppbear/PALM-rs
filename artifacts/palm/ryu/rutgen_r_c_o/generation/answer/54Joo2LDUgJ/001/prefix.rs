// Answer 0

#[test]
fn test_div100_zero() {
    let result = div100(0);
}

#[test]
fn test_div100_small_value() {
    let result = div100(99);
}

#[test]
fn test_div100_exact_hundred() {
    let result = div100(100);
}

#[test]
fn test_div100_large_value() {
    let result = div100(12345678901234567890);
}

#[test]
fn test_div100_max_value() {
    let result = div100(18446744073709551615);
}

