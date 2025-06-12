// Answer 0

#[test]
fn test_fmt_bytes_empty() {
    let bytes: &[u8] = &[];
    let unexpected = Unexpected::Bytes(bytes);
    let mut formatter = fmt::Formatter::new();
    unexpected.fmt(&mut formatter);
}

#[test]
fn test_fmt_bytes_small() {
    let bytes: &[u8] = &[1, 2, 3];
    let unexpected = Unexpected::Bytes(bytes);
    let mut formatter = fmt::Formatter::new();
    unexpected.fmt(&mut formatter);
}

#[test]
fn test_fmt_bytes_large() {
    let bytes: &[u8] = &[255; 1024];
    let unexpected = Unexpected::Bytes(bytes);
    let mut formatter = fmt::Formatter::new();
    unexpected.fmt(&mut formatter);
}

