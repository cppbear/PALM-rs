// Answer 0

#[test]
fn test_fmt_unsigned_zero() {
    let unexpected = Unexpected::Unsigned(0);
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", unexpected);
}

#[test]
fn test_fmt_unsigned_max() {
    let unexpected = Unexpected::Unsigned(u64::MAX);
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", unexpected);
}

#[test]
fn test_fmt_unsigned_small_value() {
    let unexpected = Unexpected::Unsigned(123);
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", unexpected);
}

#[test]
fn test_fmt_unsigned_large_value() {
    let unexpected = Unexpected::Unsigned(999999999);
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", unexpected);
}

#[test]
fn test_fmt_unsigned_mid_value() {
    let unexpected = Unexpected::Unsigned(5000000000);
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", unexpected);
}

