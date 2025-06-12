// Answer 0

#[test]
fn test_get_ref_empty_buf() {
    use bytes::{Buf, Bytes};
    let buf = Bytes::from(&b""[..]);
    let iter = buf.into_iter();
    let ref_inner = iter.get_ref();
}

#[test]
fn test_get_ref_one_byte_buf() {
    use bytes::{Buf, Bytes};
    let buf = Bytes::from(&b"a"[..]);
    let iter = buf.into_iter();
    let ref_inner = iter.get_ref();
}

#[test]
fn test_get_ref_256_bytes_buf() {
    use bytes::{Buf, Bytes};
    let buf = Bytes::from(vec![b'a'; 256]);
    let iter = buf.into_iter();
    let ref_inner = iter.get_ref();
}

#[test]
fn test_get_ref_65536_bytes_buf() {
    use bytes::{Buf, Bytes};
    let buf = Bytes::from(vec![b'a'; 65536]);
    let iter = buf.into_iter();
    let ref_inner = iter.get_ref();
}

#[test]
fn test_get_ref_1mb_buf() {
    use bytes::{Buf, Bytes};
    let buf = Bytes::from(vec![b'a'; 1_048_576]);
    let iter = buf.into_iter();
    let ref_inner = iter.get_ref();
}

