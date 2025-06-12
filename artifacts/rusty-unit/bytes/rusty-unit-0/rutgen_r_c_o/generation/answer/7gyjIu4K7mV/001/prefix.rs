// Answer 0

#[test]
fn test_copy_to_bytes_empty() {
    let mut bytes = Bytes::from_static(b"");
    let result = bytes.copy_to_bytes(0);
}

#[test]
fn test_copy_to_bytes_full() {
    let mut bytes = Bytes::from_static(b"Test data");
    let result = bytes.copy_to_bytes(bytes.len());
}

#[test]
fn test_copy_to_bytes_partial() {
    let mut bytes = Bytes::from_static(b"Test data");
    let result = bytes.copy_to_bytes(4);
}

#[test]
#[should_panic]
fn test_copy_to_bytes_out_of_bounds() {
    let mut bytes = Bytes::from_static(b"Test data");
    let result = bytes.copy_to_bytes(10);
}

