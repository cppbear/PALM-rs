// Answer 0

#[test]
fn test_chunk_with_empty_first_slice() {
    struct TestVecDeque {
        data: Vec<u8>,
    }

    impl TestVecDeque {
        fn as_slices(&self) -> (&[u8], &[u8]) {
            if self.data.is_empty() {
                (&[], &[])
            } else {
                let mid = self.data.len() / 2;
                (&self.data[..mid], &self.data[mid..])
            }
        }

        fn chunk(&self) -> &[u8] {
            let (s1, s2) = self.as_slices();
            if s1.is_empty() {
                s2
            } else {
                s1
            }
        }
    }

    let deque = TestVecDeque { data: vec![] };
    let result = deque.chunk();
    assert_eq!(result, &[]);
}

#[test]
fn test_chunk_with_non_empty_second_slice() {
    struct TestVecDeque {
        data: Vec<u8>,
    }

    impl TestVecDeque {
        fn as_slices(&self) -> (&[u8], &[u8]) {
            if self.data.is_empty() {
                (&[], &[])
            } else {
                let mid = self.data.len() / 2;
                (&self.data[..mid], &self.data[mid..])
            }
        }

        fn chunk(&self) -> &[u8] {
            let (s1, s2) = self.as_slices();
            if s1.is_empty() {
                s2
            } else {
                s1
            }
        }
    }

    let deque = TestVecDeque { data: vec![1, 2, 3, 4] };
    let result = deque.chunk();
    assert_eq!(result, &[3, 4]);
}

