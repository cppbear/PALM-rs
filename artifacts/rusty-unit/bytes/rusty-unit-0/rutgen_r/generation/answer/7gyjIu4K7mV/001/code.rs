// Answer 0

#[test]
fn test_copy_to_bytes_valid_length() {
    use bytes::{BytesMut, BufMut};

    let mut data = BytesMut::with_capacity(10);
    data.put(Vec::from("HelloWorld"));

    let result = data.copy_to_bytes(5);

    assert_eq!(result.len(), 5);
    assert_eq!(&result[..], b"Hello");
}

#[test]
#[should_panic]
fn test_copy_to_bytes_length_zero() {
    use bytes::{BytesMut, BufMut};

    let mut data = BytesMut::with_capacity(10);
    data.put(Vec::from("HelloWorld"));

    let _result = data.copy_to_bytes(0);
}

#[test]
#[should_panic]
fn test_copy_to_bytes_length_exceeds() {
    use bytes::{BytesMut, BufMut};

    let mut data = BytesMut::with_capacity(10);
    data.put(Vec::from("HelloWorld"));

    let _result = data.copy_to_bytes(15);
}

#[test]
fn test_copy_to_bytes_exact_length() {
    use bytes::{BytesMut, BufMut};

    let mut data = BytesMut::with_capacity(10);
    data.put(Vec::from("HelloWorld"));

    let result = data.copy_to_bytes(10);

    assert_eq!(result.len(), 10);
    assert_eq!(&result[..], b"HelloWorld");
}

