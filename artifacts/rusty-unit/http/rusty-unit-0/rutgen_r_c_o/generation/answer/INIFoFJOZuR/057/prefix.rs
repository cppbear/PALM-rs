// Answer 0

#[test]
fn test_from_bytes_len_3_not_get() {
    let input1 = b"PUT";
    let _ = Method::from_bytes(input1);

    let input2 = b"XYZ";
    let _ = Method::from_bytes(input2);
}

