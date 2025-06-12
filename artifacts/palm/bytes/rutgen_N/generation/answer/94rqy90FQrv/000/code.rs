// Answer 0

#[test]
fn test_chunk() {
    struct MyBytes {
        data: Vec<u8>,
    }

    impl MyBytes {
        fn new(data: Vec<u8>) -> Self {
            MyBytes { data }
        }

        fn as_slice(&self) -> &[u8] {
            &self.data
        }

        fn chunk(&self) -> &[u8] {
            self.as_slice()
        }
    }

    let bytes = MyBytes::new(vec![1, 2, 3, 4, 5]);
    let chunked = bytes.chunk();
    
    assert_eq!(chunked, &[1, 2, 3, 4, 5]);
}

#[test]
fn test_chunk_empty() {
    struct MyBytes {
        data: Vec<u8>,
    }

    impl MyBytes {
        fn new(data: Vec<u8>) -> Self {
            MyBytes { data }
        }

        fn as_slice(&self) -> &[u8] {
            &self.data
        }

        fn chunk(&self) -> &[u8] {
            self.as_slice()
        }
    }

    let bytes = MyBytes::new(vec![]);
    let chunked = bytes.chunk();
    
    assert_eq!(chunked, &[]);
}

