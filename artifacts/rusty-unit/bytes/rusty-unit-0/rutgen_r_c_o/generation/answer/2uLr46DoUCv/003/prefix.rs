// Answer 0

#[test]
fn test_truncate_case_1() {
    let mut buf = Bytes::from_static(b"hello world");
    buf.truncate(5);
}

#[test]
fn test_truncate_case_2() {
    let mut buf = Bytes::from_static(b"hello world");
    buf.truncate(1);
}

#[test]
fn test_truncate_case_3() {
    let mut buf = Bytes::from_static(b"hello world");
    buf.truncate(10);
}

#[test]
fn test_truncate_case_4() {
    let mut buf = Bytes::from_static(b"hello world");
    buf.truncate(9);
}

#[test]
fn test_truncate_case_5() {
    let mut buf = Bytes::from_static(b"hello world");
    buf.truncate(7);
}

