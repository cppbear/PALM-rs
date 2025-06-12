// Answer 0

#[test]
fn test_put_with_empty_buf() {
    struct EmptyBuf;

    impl Buf for EmptyBuf {
        fn chunk(&self) -> &[u8] {
            &[]
        }

        fn has_remaining(&self) -> bool {
            false
        }

        fn advance(&mut self, _: usize) {}
    }

    let mut destination: Vec<u8> = Vec::new();
    let src = EmptyBuf;

    destination.put(src);
    
    assert_eq!(destination.len(), 0);
}

#[test]
fn test_put_with_single_chunk_buf() {
    struct SingleChunkBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl SingleChunkBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl Buf for SingleChunkBuf {
        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn has_remaining(&self) -> bool {
            self.position < self.data.len()
        }

        fn advance(&mut self, n: usize) {
            self.position += n;
        }
    }

    let mut destination: Vec<u8> = Vec::new();
    let src = SingleChunkBuf::new(vec![1, 2, 3]);

    destination.put(src);
    
    assert_eq!(destination, vec![1, 2, 3]);
} 

#[test]
fn test_put_with_multiple_chunks_buf() {
    struct MultiChunkBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl MultiChunkBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl Buf for MultiChunkBuf {
        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn has_remaining(&self) -> bool {
            self.position < self.data.len()
        }

        fn advance(&mut self, n: usize) {
            self.position += n;
        }
    }

    let mut destination: Vec<u8> = Vec::new();
    let src = MultiChunkBuf::new(vec![1, 2, 3, 4, 5]);

    destination.put(src);
    
    assert_eq!(destination, vec![1, 2, 3, 4, 5]);
}

