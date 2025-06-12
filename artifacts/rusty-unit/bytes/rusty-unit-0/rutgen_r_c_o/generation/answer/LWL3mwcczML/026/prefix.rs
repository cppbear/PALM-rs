// Answer 0

#[test]
fn test_empty_bytes_ref() {
    let data = b"";
    let bytes_ref = BytesRef(data);
    let mut formatter = Formatter::new();
    bytes_ref.fmt(&mut formatter);
}

#[test]
fn test_non_printable_bytes() {
    let data = &[0x01, 0x02, 0x03];
    let bytes_ref = BytesRef(data);
    let mut formatter = Formatter::new();
    bytes_ref.fmt(&mut formatter);
}

#[test]
fn test_edge_case_with_non_printable_and_empty() {
    let data = b"\x00\x01\x02\x03";
    let bytes_ref = BytesRef(data);
    let mut formatter = Formatter::new();
    bytes_ref.fmt(&mut formatter);
}

