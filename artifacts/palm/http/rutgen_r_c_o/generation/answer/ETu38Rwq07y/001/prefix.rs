// Answer 0

#[test]
fn test_debug_fmt_valid_range_100() {
    let status_code = StatusCode(NonZeroU16::new(100).unwrap());
    let mut formatter = fmt::Formatter::new();
    let _ = status_code.fmt(&mut formatter);
}

#[test]
fn test_debug_fmt_valid_range_200() {
    let status_code = StatusCode(NonZeroU16::new(200).unwrap());
    let mut formatter = fmt::Formatter::new();
    let _ = status_code.fmt(&mut formatter);
}

#[test]
fn test_debug_fmt_valid_range_300() {
    let status_code = StatusCode(NonZeroU16::new(300).unwrap());
    let mut formatter = fmt::Formatter::new();
    let _ = status_code.fmt(&mut formatter);
}

#[test]
fn test_debug_fmt_valid_range_400() {
    let status_code = StatusCode(NonZeroU16::new(400).unwrap());
    let mut formatter = fmt::Formatter::new();
    let _ = status_code.fmt(&mut formatter);
}

#[test]
fn test_debug_fmt_valid_range_500() {
    let status_code = StatusCode(NonZeroU16::new(500).unwrap());
    let mut formatter = fmt::Formatter::new();
    let _ = status_code.fmt(&mut formatter);
}

#[test]
fn test_debug_fmt_valid_range_511() {
    let status_code = StatusCode(NonZeroU16::new(511).unwrap());
    let mut formatter = fmt::Formatter::new();
    let _ = status_code.fmt(&mut formatter);
}

