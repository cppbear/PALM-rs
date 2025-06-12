// Answer 0

#[test]
fn test_fmt_with_newline() {
    let bytes_ref = BytesRef(&[b'\n']);
    let mut formatter = Formatter::new();
    let _ = bytes_ref.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_carriage_return() {
    let bytes_ref = BytesRef(&[b'\r']);
    let mut formatter = Formatter::new();
    let _ = bytes_ref.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_multiple_bytes() {
    let bytes_ref = BytesRef(&[b'a', b'b', b'\n', b'\r']);
    let mut formatter = Formatter::new();
    let _ = bytes_ref.fmt(&mut formatter);
}

#[test]
#[should_panic]
fn test_fmt_with_tab() {
    let bytes_ref = BytesRef(&[b'\t']);
    let mut formatter = Formatter::new();
    let _ = bytes_ref.fmt(&mut formatter);
}

#[test]
#[should_panic]
fn test_fmt_with_invalid_write() {
    let bytes_ref = BytesRef(&[b'\t', b'\n']);
    let mut formatter = Formatter::new();
    let _ = bytes_ref.fmt(&mut formatter);
}

