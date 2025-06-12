// Answer 0

#[test]
fn test_chunk_mut_with_remaining_mut_a() {
    struct MockSlice {
        data: Vec<u8>,
        index: usize,
    }

    impl MockSlice {
        fn new(data: Vec<u8>) -> Self {
            MockSlice { data, index: 0 }
        }

        fn has_remaining_mut(&self) -> bool {
            self.index < self.data.len()
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            &mut self.data[self.index..self.data.len()]
        }
    }

    struct BufferChain {
        a: MockSlice,
        b: MockSlice,
    }

    impl BufferChain {
        fn new(a: MockSlice, b: MockSlice) -> Self {
            BufferChain { a, b }
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            if self.a.has_remaining_mut() {
                self.a.chunk_mut()
            } else {
                self.b.chunk_mut()
            }
        }
    }

    let mut buffer_chain = BufferChain::new(MockSlice::new(vec![1, 2, 3]), MockSlice::new(vec![4, 5, 6]));

    let chunk = buffer_chain.chunk_mut();

    assert_eq!(chunk, &[1, 2, 3]);
}

#[test]
fn test_chunk_mut_without_remaining_mut_a() {
    struct MockSlice {
        data: Vec<u8>,
        index: usize,
    }

    impl MockSlice {
        fn new(data: Vec<u8>) -> Self {
            MockSlice { data, index: 0 }
        }

        fn has_remaining_mut(&self) -> bool {
            self.index >= self.data.len() // Simulates no remaining mutable elements
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            &mut self.data[self.index..self.data.len()]
        }
    }

    struct BufferChain {
        a: MockSlice,
        b: MockSlice,
    }

    impl BufferChain {
        fn new(a: MockSlice, b: MockSlice) -> Self {
            BufferChain { a, b }
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            if self.a.has_remaining_mut() {
                self.a.chunk_mut()
            } else {
                self.b.chunk_mut()
            }
        }
    }

    let mut buffer_chain = BufferChain::new(MockSlice::new(vec![]), MockSlice::new(vec![4, 5, 6]));

    let chunk = buffer_chain.chunk_mut();

    assert_eq!(chunk, &[4, 5, 6]);
}

