// Answer 0

#[test]
fn test_bytes_ref_upper_hex_empty() {
    let bytes_ref = BytesRef(&[]);
    let mut formatter = Formatter::default();
    let _ = bytes_ref.fmt(&mut formatter);
}

#[test]
fn test_bytes_ref_upper_hex_single_byte() {
    let bytes_ref = BytesRef(&[0]);
    let mut formatter = Formatter::default();
    let _ = bytes_ref.fmt(&mut formatter);

    let bytes_ref = BytesRef(&[255]);
    let _ = bytes_ref.fmt(&mut formatter);
}

#[test]
fn test_bytes_ref_upper_hex_multiple_bytes() {
    let bytes_ref = BytesRef(&[0, 1, 2, 3, 4, 5, 255]);
    let mut formatter = Formatter::default();
    let _ = bytes_ref.fmt(&mut formatter);
}

#[test]
fn test_bytes_ref_upper_hex_full_range() {
    let bytes_ref = BytesRef(&(0..=255).collect::<Vec<u8>>());
    let mut formatter = Formatter::default();
    let _ = bytes_ref.fmt(&mut formatter);
}

#[test]
#[should_panic]
fn test_bytes_ref_upper_hex_invalid_write() {
    let bytes_ref = BytesRef(&[128]);
    let mut formatter = Formatter::default();
    let result = bytes_ref.fmt(&mut formatter);
    if result.is_err() {
        panic!();
    }
}

