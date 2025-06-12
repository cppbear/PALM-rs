// Answer 0

#[test]
fn test_split_to_must_use() {
    use crate::Bytes;

    let mut b1 = Bytes::from("hello world");
    let result = b1.split_to(6);
    assert_eq!(result.as_ref(), "hello ");
    assert_eq!(b1.as_ref(), "world");
} 

#[test]
#[should_panic]
fn test_split_to_must_use_panic() {
    use crate::Bytes;

    let b1 = Bytes::from("hello world");
    // This will panic because the result is not used
    b1.split_to(6);
}

