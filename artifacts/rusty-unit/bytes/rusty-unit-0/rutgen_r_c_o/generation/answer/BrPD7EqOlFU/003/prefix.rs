// Answer 0

#[test]
fn test_bytes_ref_empty() {
    let bytes_ref = BytesRef(&[]);
    let mut formatter = Formatter::new();
    let _ = bytes_ref.fmt(&mut formatter);
}

#[test]
fn test_bytes_ref_single_zero() {
    let bytes_ref = BytesRef(&[0]);
    let mut formatter = Formatter::new();
    let _ = bytes_ref.fmt(&mut formatter);
}

#[test]
fn test_bytes_ref_single_max() {
    let bytes_ref = BytesRef(&[255]);
    let mut formatter = Formatter::new();
    let _ = bytes_ref.fmt(&mut formatter);
}

#[test]
fn test_bytes_ref_multiple() {
    let bytes_ref = BytesRef(&[0, 128, 255]);
    let mut formatter = Formatter::new();
    let _ = bytes_ref.fmt(&mut formatter);
}

