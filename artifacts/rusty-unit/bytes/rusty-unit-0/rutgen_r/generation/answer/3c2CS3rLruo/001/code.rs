// Answer 0

#[test]
fn test_chunk_mut_with_limit_below_inner_length() {
    struct TestStruct {
        inner: Vec<u8>,
        limit: usize,
    }

    impl TestStruct {
        fn new(inner: Vec<u8>, limit: usize) -> Self {
            Self { inner, limit }
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            let bytes = &mut self.inner;
            let end = std::cmp::min(bytes.len(), self.limit);
            &mut bytes[..end]
        }
    }

    let mut test_struct = TestStruct::new(vec![1, 2, 3, 4, 5], 3);
    let chunk = test_struct.chunk_mut();
    assert_eq!(chunk, &[1, 2, 3]);
}

#[test]
fn test_chunk_mut_with_limit_equal_to_inner_length() {
    struct TestStruct {
        inner: Vec<u8>,
        limit: usize,
    }

    impl TestStruct {
        fn new(inner: Vec<u8>, limit: usize) -> Self {
            Self { inner, limit }
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            let bytes = &mut self.inner;
            let end = std::cmp::min(bytes.len(), self.limit);
            &mut bytes[..end]
        }
    }

    let mut test_struct = TestStruct::new(vec![1, 2, 3], 3);
    let chunk = test_struct.chunk_mut();
    assert_eq!(chunk, &[1, 2, 3]);
}

#[test]
fn test_chunk_mut_with_limit_greater_than_inner_length() {
    struct TestStruct {
        inner: Vec<u8>,
        limit: usize,
    }

    impl TestStruct {
        fn new(inner: Vec<u8>, limit: usize) -> Self {
            Self { inner, limit }
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            let bytes = &mut self.inner;
            let end = std::cmp::min(bytes.len(), self.limit);
            &mut bytes[..end]
        }
    }

    let mut test_struct = TestStruct::new(vec![1, 2], 4);
    let chunk = test_struct.chunk_mut();
    assert_eq!(chunk, &[1, 2]);
}

#[should_panic]
#[test]
fn test_chunk_mut_with_empty_inner() {
    struct TestStruct {
        inner: Vec<u8>,
        limit: usize,
    }

    impl TestStruct {
        fn new(inner: Vec<u8>, limit: usize) -> Self {
            Self { inner, limit }
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            let bytes = &mut self.inner;
            let end = std::cmp::min(bytes.len(), self.limit);
            &mut bytes[..end]  // This will panic if inner is empty and limit is more than 0.
        }
    }

    let mut test_struct = TestStruct::new(Vec::new(), 1);
    test_struct.chunk_mut();
}

