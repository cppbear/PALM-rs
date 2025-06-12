// Answer 0

#[test]
fn test_first_mut_with_non_empty_buf() {
    struct TestBuf {
        a: Vec<u8>,
    }

    impl TestBuf {
        pub fn new(data: Vec<u8>) -> Self {
            TestBuf { a: data }
        }
    }
    
    let mut buf = TestBuf::new(vec![1, 2, 3, 4, 5]);
    let first = buf.first_mut();
    
    assert_eq!(first, &mut vec![1, 2, 3, 4, 5][..]);
}

#[test]
fn test_first_mut_with_single_element_buf() {
    struct TestBuf {
        a: Vec<u8>,
    }

    impl TestBuf {
        pub fn new(data: Vec<u8>) -> Self {
            TestBuf { a: data }
        }
    }
    
    let mut buf = TestBuf::new(vec![10]);
    let first = buf.first_mut();
    
    assert_eq!(first, &mut vec![10][..]);
}

#[test]
fn test_first_mut_with_empty_buf() {
    struct TestBuf {
        a: Vec<u8>,
    }

    impl TestBuf {
        pub fn new(data: Vec<u8>) -> Self {
            TestBuf { a: data }
        }
    }

    let mut buf = TestBuf::new(Vec::new());
    let first = buf.first_mut();
    
    first.push(1); // Test to see if we can mutate the empty buffer
    assert_eq!(first, &mut vec![1][..]);
} 

#[should_panic]
fn test_first_mut_on_uninitialized_buf() {
    struct TestBuf {
        a: Option<Vec<u8>>,
    }

    impl TestBuf {
        pub fn new() -> Self {
            TestBuf { a: None }
        }
    }
    
    let mut buf = TestBuf::new();
    let _ = buf.first_mut(); // This should panic because `a` is None
}

