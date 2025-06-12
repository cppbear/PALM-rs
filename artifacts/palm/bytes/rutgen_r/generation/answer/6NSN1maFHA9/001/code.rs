// Answer 0

#[test]
fn test_chunk_mut_with_full_capacity() {
    struct BytesMut {
        buf: Vec<u8>,
        len: usize,
        capacity: usize,
    }

    impl BytesMut {
        fn new() -> Self {
            BytesMut {
                buf: Vec::with_capacity(64),
                len: 0,
                capacity: 64,
            }
        }
        
        fn reserve(&mut self, additional: usize) {
            self.buf.reserve(additional);
            self.capacity += additional;
        }
        
        fn len(&self) -> usize {
            self.len
        }
        
        fn capacity(&self) -> usize {
            self.capacity
        }
        
        fn spare_capacity_mut(&mut self) -> &mut Vec<u8> {
            &mut self.buf[self.len..]
        }
        
        fn chunk_mut(&mut self) -> &mut [u8] {
            if self.capacity() == self.len() {
                self.reserve(64);
            }
            self.spare_capacity_mut().as_mut()
        }
    }

    let mut bytes_mut = BytesMut::new();
    bytes_mut.len = 64; // Set len equal to capacity to trigger the conditional

    let chunk = bytes_mut.chunk_mut();
    assert_eq!(chunk.len(), 64); // Check the length of the reserved space
}

