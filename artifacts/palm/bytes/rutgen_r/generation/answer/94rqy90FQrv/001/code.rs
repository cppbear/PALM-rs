// Answer 0

#[test]
fn test_chunk_empty_slice() {
    struct Bytes {
        data: Vec<u8>,
    }

    impl Bytes {
        fn as_slice(&self) -> &[u8] {
            &self.data
        }

        fn chunk(&self) -> &[u8] {
            self.as_slice()
        }
    }

    let bytes = Bytes { data: Vec::new() };
    assert_eq!(bytes.chunk(), &[]);
}

#[test]
fn test_chunk_non_empty_slice() {
    struct Bytes {
        data: Vec<u8>,
    }

    impl Bytes {
        fn as_slice(&self) -> &[u8] {
            &self.data
        }

        fn chunk(&self) -> &[u8] {
            self.as_slice()
        }
    }

    let bytes = Bytes { data: vec![1, 2, 3, 4, 5] };
    assert_eq!(bytes.chunk(), &[1, 2, 3, 4, 5]);
}

#[test]
fn test_chunk_single_element_slice() {
    struct Bytes {
        data: Vec<u8>,
    }

    impl Bytes {
        fn as_slice(&self) -> &[u8] {
            &self.data
        }

        fn chunk(&self) -> &[u8] {
            self.as_slice()
        }
    }

    let bytes = Bytes { data: vec![42] };
    assert_eq!(bytes.chunk(), &[42]);
}

#[test]
fn test_chunk_large_slice() {
    struct Bytes {
        data: Vec<u8>,
    }

    impl Bytes {
        fn as_slice(&self) -> &[u8] {
            &self.data
        }

        fn chunk(&self) -> &[u8] {
            self.as_slice()
        }
    }

    let bytes = Bytes { data: (0..255).collect::<Vec<u8>>() };
    assert_eq!(bytes.chunk(), &(0..255).collect::<Vec<u8>>()[..]);
}

