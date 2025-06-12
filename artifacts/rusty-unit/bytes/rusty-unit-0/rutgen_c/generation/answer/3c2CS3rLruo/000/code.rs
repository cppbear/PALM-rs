// Answer 0

#[test]
fn test_chunk_mut_within_limit() {
    struct TestBuf {
        data: Vec<u8>,
        cursor: usize,
    }

    unsafe impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.cursor
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            let slice = &mut self.data[self.cursor..];
            UninitSlice::new(slice)
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.cursor += cnt;
        }
    }

    let mut buf = TestBuf {
        data: vec![0; 10],
        cursor: 0,
    };
    
    let limit = 5;
    let mut limited_buf = buf.limit(limit);
    
    let chunk = limited_buf.chunk_mut();
    assert_eq!(chunk.len(), 5);
}

#[test]
fn test_chunk_mut_exact_limit() {
    struct TestBuf {
        data: Vec<u8>,
        cursor: usize,
    }

    unsafe impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.cursor
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            let slice = &mut self.data[self.cursor..];
            UninitSlice::new(slice)
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.cursor += cnt;
        }
    }

    let mut buf = TestBuf {
        data: vec![0; 5],
        cursor: 0,
    };
    
    let limit = 5;
    let mut limited_buf = buf.limit(limit);
    
    let chunk = limited_buf.chunk_mut();
    assert_eq!(chunk.len(), 5);
}

#[test]
fn test_chunk_mut_beyond_limit() {
    struct TestBuf {
        data: Vec<u8>,
        cursor: usize,
    }

    unsafe impl BufMut for TestBuf {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.cursor
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            let slice = &mut self.data[self.cursor..];
            UninitSlice::new(slice)
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.cursor += cnt;
        }
    }

    let mut buf = TestBuf {
        data: vec![0; 8],
        cursor: 0,
    };
    
    let limit = 10;
    let mut limited_buf = buf.limit(limit);
    
    let chunk = limited_buf.chunk_mut();
    assert_eq!(chunk.len(), 8);
}

