// Answer 0

#[test]
fn test_chunk_mut_with_available_capacity() {
    struct BytesMut {
        buffer: Vec<u8>,
        len: usize,
        capacity: usize,
    }

    impl BytesMut {
        fn new() -> Self {
            Self {
                buffer: Vec::new(),
                len: 0,
                capacity: 0,
            }
        }

        fn reserve(&mut self, additional: usize) {
            self.buffer.resize(self.len + additional, 0);
            self.capacity = self.len + additional;
        }

        fn len(&self) -> usize {
            self.len
        }

        fn capacity(&self) -> usize {
            self.capacity
        }

        fn spare_capacity_mut(&mut self) -> &mut [u8] {
            &mut self.buffer[self.len..self.capacity]
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            if self.capacity() == self.len() {
                self.reserve(64);
            }
            self.spare_capacity_mut()
        }
    }

    let mut bytes_mut = BytesMut::new();
    bytes_mut.reserve(100); // Pre-filling with a capacity of 100
    bytes_mut.len = 50; // Setting the length to 50

    let chunk = bytes_mut.chunk_mut();
    assert!(chunk.len() > 0); // Ensuring that we have spare capacity
    assert_eq!(bytes_mut.len(), 50); // Length should remain the same
    assert!(bytes_mut.capacity() >= 100); // Capacity should still be adequate
}

