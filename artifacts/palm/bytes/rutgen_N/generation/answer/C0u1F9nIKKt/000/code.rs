// Answer 0

#[test]
fn test_reader_new() {
    struct TestBuffer {
        data: Vec<u8>,
    }
    
    impl TestBuffer {
        fn new(data: Vec<u8>) -> Self {
            TestBuffer { data }
        }
    }

    struct Reader<B> {
        buf: B,
    }

    let buffer = TestBuffer::new(vec![1, 2, 3, 4]);
    let reader = new(buffer);
    
    assert_eq!(reader.buf.data, vec![1, 2, 3, 4]);
}

