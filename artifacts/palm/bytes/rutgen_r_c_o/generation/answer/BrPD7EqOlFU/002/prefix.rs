// Answer 0

#[test]
fn test_empty_slice() {
    let data = b"";
    let bytes_ref = BytesRef(data);
    let mut formatter = Formatter::new();
    let _ = bytes_ref.fmt(&mut formatter);
}

#[test]
fn test_valid_data_slice() {
    let data = b"\x00\xFF";
    let bytes_ref = BytesRef(data);
    let mut formatter = Formatter::new();
    let _ = bytes_ref.fmt(&mut formatter);
}

#[test]
fn test_large_data_slice() {
    let data: Vec<u8> = (0..1000).map(|x| (x % 256) as u8).collect();
    let bytes_ref = BytesRef(&data);
    let mut formatter = Formatter::new();
    let _ = bytes_ref.fmt(&mut formatter);
}

