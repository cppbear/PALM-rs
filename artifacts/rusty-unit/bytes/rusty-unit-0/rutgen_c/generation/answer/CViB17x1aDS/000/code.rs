// Answer 0

#[test]
fn test_split_off_must_use() {
    use crate::bytes::Bytes;

    #[allow(unused_must_use)]
    {
        let mut b1 = Bytes::from("hello world");
        let _result = b1.split_off(6);
    }
}

#[test]
#[should_panic]
fn test_split_off_out_of_bounds() {
    use crate::bytes::Bytes;

    let mut b1 = Bytes::from("hello world");
    let result = b1.split_off(20); // Should panic or trigger an error since the index is out of bounds
    assert!(result.is_err());
}

