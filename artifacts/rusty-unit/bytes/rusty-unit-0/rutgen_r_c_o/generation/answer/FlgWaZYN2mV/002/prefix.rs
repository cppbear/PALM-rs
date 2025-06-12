// Answer 0

#[test]
fn test_split_off_normal_case() {
    let mut bytes = Bytes::from_static(b"hello world");
    let result = bytes.split_off(5);
}

#[test]
fn test_split_off_edge_case_1() {
    let mut bytes = Bytes::from_static(b"hello world");
    let result = bytes.split_off(1);
}

#[test]
fn test_split_off_edge_case_2() {
    let mut bytes = Bytes::from_static(b"hello world");
    let result = bytes.split_off(9);
}

#[test]
#[should_panic] 
fn test_split_off_panic_case_too_high() {
    let mut bytes = Bytes::from_static(b"hello world");
    let result = bytes.split_off(11);
}

#[test]
#[should_panic] 
fn test_split_off_panic_case_zero() {
    let mut bytes = Bytes::from_static(b"hello world");
    let result = bytes.split_off(0);
}

