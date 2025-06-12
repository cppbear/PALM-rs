// Answer 0

#[test]
fn test_chunk_mut_when_capacity_is_less_than_len() {
    struct BytesMut {
        data: Vec<u8>,
        capacity: usize,
    }

    impl BytesMut {
        fn new() -> Self {
            BytesMut { data: Vec::new(), capacity: 0 }
        }

        fn capacity(&self) -> usize {
            self.capacity
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn reserve(&mut self, additional: usize) {
            self.capacity += additional;
            self.data.reserve(additional);
        }

        fn spare_capacity_mut(&mut self) -> &mut [u8] {
            let current_len = self.len();
            let spare_length = self.capacity - current_len;
            &mut self.data[current_len..current_len + spare_length]
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            if self.capacity() == self.len() {
                self.reserve(64);
            }
            self.spare_capacity_mut()
        }
    }

    let mut bytes = BytesMut::new();
    bytes.data = vec![1, 2, 3, 4, 5];
    bytes.capacity = 5;

    let chunk = bytes.chunk_mut();
    assert!(chunk.len() >= 64); // We expect at least 64 bytes of capacity to be available
}

#[test]
fn test_chunk_mut_when_capacity_equals_len() {
    struct BytesMut {
        data: Vec<u8>,
        capacity: usize,
    }

    impl BytesMut {
        fn new() -> Self {
            BytesMut { data: Vec::new(), capacity: 0 }
        }

        fn capacity(&self) -> usize {
            self.capacity
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn reserve(&mut self, additional: usize) {
            self.capacity += additional;
            self.data.reserve(additional);
        }

        fn spare_capacity_mut(&mut self) -> &mut [u8] {
            let current_len = self.len();
            let spare_length = self.capacity - current_len;
            &mut self.data[current_len..current_len + spare_length]
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            if self.capacity() == self.len() {
                self.reserve(64);
            }
            self.spare_capacity_mut()
        }
    }

    let mut bytes = BytesMut::new();
    bytes.data = vec![1, 2, 3, 4, 5];
    bytes.capacity = 5;

    let chunk = bytes.chunk_mut();
    assert!(chunk.len() >= 64); // We expect at least 64 bytes of capacity to be available
}

#[test]
fn test_chunk_mut_with_empty_initialization() {
    struct BytesMut {
        data: Vec<u8>,
        capacity: usize,
    }

    impl BytesMut {
        fn new() -> Self {
            BytesMut { data: Vec::new(), capacity: 0 }
        }

        fn capacity(&self) -> usize {
            self.capacity
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn reserve(&mut self, additional: usize) {
            self.capacity += additional;
            self.data.reserve(additional);
        }

        fn spare_capacity_mut(&mut self) -> &mut [u8] {
            let current_len = self.len();
            let spare_length = self.capacity - current_len;
            &mut self.data[current_len..current_len + spare_length]
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            if self.capacity() == self.len() {
                self.reserve(64);
            }
            self.spare_capacity_mut()
        }
    }

    let mut bytes = BytesMut::new();

    let chunk = bytes.chunk_mut();
    assert!(chunk.len() >= 64); // We expect at least 64 bytes of capacity to be available
}

