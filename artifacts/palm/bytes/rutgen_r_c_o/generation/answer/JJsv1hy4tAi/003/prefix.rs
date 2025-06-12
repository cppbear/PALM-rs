// Answer 0

#[test]
fn test_fmt_empty_slice() {
    let bytes_ref = BytesRef(&[]);
    let mut formatter = Formatter::new();
    bytes_ref.fmt(&mut formatter).unwrap();
}

#[test]
fn test_fmt_small_slice() {
    let bytes_ref = BytesRef(&[0u8, 1u8, 2u8, 3u8, 4u8]);
    let mut formatter = Formatter::new();
    bytes_ref.fmt(&mut formatter).unwrap();
}

#[test]
fn test_fmt_large_slice() {
    let bytes_ref = BytesRef(&(0..256).map(|x| x as u8).collect::<Vec<u8>>());
    let mut formatter = Formatter::new();
    bytes_ref.fmt(&mut formatter).unwrap();
}

#[test]
fn test_fmt_single_max_capacity() {
    let bytes_ref = BytesRef(&[u8::MAX]);
    let mut formatter = Formatter::new();
    bytes_ref.fmt(&mut formatter).unwrap();
}

#[test]
fn test_fmt_single_zero() {
    let bytes_ref = BytesRef(&[0u8]);
    let mut formatter = Formatter::new();
    bytes_ref.fmt(&mut formatter).unwrap();
}

