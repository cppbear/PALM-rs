// Answer 0

#[test]
fn test_chunk_mut_empty_buffer() {
    struct TestBuf {
        data: Vec<u8>,
    }

    impl TestBuf {
        fn chunk_mut(&mut self) -> &mut [u8] {
            &mut self.data
        }
    }

    let mut buf = TestBuf { data: Vec::new() };
    let chunk = buf.chunk_mut();
    assert!(chunk.is_empty());
}

#[test]
fn test_chunk_mut_non_empty_buffer() {
    struct TestBuf {
        data: Vec<u8>,
    }

    impl TestBuf {
        fn chunk_mut(&mut self) -> &mut [u8] {
            &mut self.data
        }
    }

    let mut buf = TestBuf { data: vec![1, 2, 3] };
    let chunk = buf.chunk_mut();
    assert_eq!(chunk, [1, 2, 3]);
}

#[test]
fn test_chunk_mut_single_element_buffer() {
    struct TestBuf {
        data: Vec<u8>,
    }

    impl TestBuf {
        fn chunk_mut(&mut self) -> &mut [u8] {
            &mut self.data
        }
    }

    let mut buf = TestBuf { data: vec![42] };
    let chunk = buf.chunk_mut();
    assert_eq!(chunk, [42]);
}

#[test]
#[should_panic]
fn test_chunk_mut_panic_on_mutation() {
    struct TestBuf {
        data: Vec<u8>,
    }

    impl TestBuf {
        fn chunk_mut(&mut self) -> &mut [u8] {
            &mut self.data
        }
    }

    let mut buf = TestBuf { data: vec![1] };
    let _chunk = buf.chunk_mut();
    buf.data.push(2); // This should panic due to interior mutability.
}

