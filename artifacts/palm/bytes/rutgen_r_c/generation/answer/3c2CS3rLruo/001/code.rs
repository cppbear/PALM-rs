// Answer 0

#[test]
fn test_chunk_mut_within_limit() {
    struct TestBufMut {
        data: Vec<u8>,
        position: usize,
    }
    
    unsafe impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }
        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            let slice = &mut self.data[self.position..];
            let uninit_slice = UninitSlice::new(slice);
            uninit_slice
        }
    }
    
    let mut buffer = TestBufMut { data: vec![0; 10], position: 0 };
    let limit = 5;
    let limit_buf = Limit { inner: buffer, limit };
    let chunk = limit_buf.chunk_mut();
    assert_eq!(chunk.len(), limit);
}

#[test]
#[should_panic]
fn test_chunk_mut_exceeds_limit() {
    struct TestBufMut {
        data: Vec<u8>,
        position: usize,
    }
    
    unsafe impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.data.len() - self.position
        }
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }
        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            let slice = &mut self.data[self.position..];
            let uninit_slice = UninitSlice::new(slice);
            uninit_slice
        }
    }
    
    let mut buffer = TestBufMut { data: vec![0; 5], position: 0 };
    let limit = 10; // Exceeds length of data
    let limit_buf = Limit { inner: buffer, limit };
    let _chunk = limit_buf.chunk_mut(); // This should panic
}

