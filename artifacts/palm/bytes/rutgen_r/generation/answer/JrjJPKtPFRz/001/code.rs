// Answer 0

#[test]
fn test_read_with_full_buffer() {
    struct MockBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl MockBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }
        
        fn copy_to_slice(src: &mut MockBuf, dst: &mut [u8]) {
            let bytes_to_copy = dst.len().min(src.remaining());
            dst[..bytes_to_copy].copy_from_slice(&src.data[src.pos..src.pos + bytes_to_copy]);
            src.pos += bytes_to_copy;
        }
    }

    let mut buf = MockBuf {
        data: vec![1, 2, 3, 4, 5],
        pos: 0,
    };
    
    let mut dst = [0u8; 5];
    let result = read(&mut buf, &mut dst);

    assert_eq!(result, Ok(5));
    assert_eq!(dst, [1, 2, 3, 4, 5]);
}

#[test]
fn test_read_with_partial_buffer() {
    struct MockBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl MockBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }
        
        fn copy_to_slice(src: &mut MockBuf, dst: &mut [u8]) {
            let bytes_to_copy = dst.len().min(src.remaining());
            dst[..bytes_to_copy].copy_from_slice(&src.data[src.pos..src.pos + bytes_to_copy]);
            src.pos += bytes_to_copy;
        }
    }

    let mut buf = MockBuf {
        data: vec![1, 2, 3],
        pos: 0,
    };
    
    let mut dst = [0u8; 5];
    let result = read(&mut buf, &mut dst);

    assert_eq!(result, Ok(3));
    assert_eq!(dst, [1, 2, 3, 0, 0]);
}

#[test]
fn test_read_with_empty_buffer() {
    struct MockBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl MockBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.pos
        }
        
        fn copy_to_slice(src: &mut MockBuf, dst: &mut [u8]) {
            let bytes_to_copy = dst.len().min(src.remaining());
            dst[..bytes_to_copy].copy_from_slice(&src.data[src.pos..src.pos + bytes_to_copy]);
            src.pos += bytes_to_copy;
        }
    }

    let mut buf = MockBuf {
        data: vec![],
        pos: 0,
    };
    
    let mut dst = [0u8; 5];
    let result = read(&mut buf, &mut dst);

    assert_eq!(result, Ok(0));
    assert_eq!(dst, [0, 0, 0, 0, 0]);
}

