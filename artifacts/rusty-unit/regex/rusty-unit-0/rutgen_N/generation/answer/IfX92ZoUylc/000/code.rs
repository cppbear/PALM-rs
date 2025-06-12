// Answer 0

#[test]
fn test_find_empty_dense() {
    struct TestStruct {
        dense: Vec<u8>,
    }

    let test_instance = TestStruct { dense: Vec::new() };
    assert_eq!(test_instance.find(b"sample text"), None);
}

#[test]
fn test_find_one_byte_dense() {
    struct TestStruct {
        dense: Vec<u8>,
    }

    let test_instance = TestStruct { dense: vec![b'a'] };
    assert_eq!(test_instance.find(b"sample text"), None);
    assert_eq!(test_instance.find(b"a"), Some(0));
    assert_eq!(test_instance.find(b"abc"), Some(0));
}

#[test]
fn test_find_two_byte_dense() {
    struct TestStruct {
        dense: Vec<u8>,
    }

    let test_instance = TestStruct { dense: vec![b'a', b'b'] };
    assert_eq!(test_instance.find(b"sample text"), None);
    assert_eq!(test_instance.find(b"ab"), Some(0));
    assert_eq!(test_instance.find(b"abc"), Some(0));
    assert_eq!(test_instance.find(b"ba"), None);
}

#[test]
fn test_find_three_byte_dense() {
    struct TestStruct {
        dense: Vec<u8>,
    }

    let test_instance = TestStruct { dense: vec![b'a', b'b', b'c'] };
    assert_eq!(test_instance.find(b"sample text"), None);
    assert_eq!(test_instance.find(b"abc"), Some(0));
    assert_eq!(test_instance.find(b"bca"), None);
    assert_eq!(test_instance.find(b"cab"), None);
}

#[test]
fn test_find_greater_than_three_byte_dense() {
    struct TestStruct {
        dense: Vec<u8>,
    }

    let test_instance = TestStruct { dense: vec![b'a', b'b', b'c', b'd'] };
    assert_eq!(test_instance.find(b"sample text"), None);
    assert_eq!(test_instance.find(b"bc"), None);
    assert_eq!(test_instance.find(b"abc"), Some(0));
    assert_eq!(test_instance.find(b"cde"), None);
}

