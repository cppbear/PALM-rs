// Answer 0

#[test]
fn test_chunk_mut_within_limit() {
    struct Inner {
        data: Vec<u8>,
    }

    impl Inner {
        fn chunk_mut(&mut self) -> &mut [u8] {
            &mut self.data
        }
    }

    struct Limited {
        inner: Inner,
        limit: usize,
    }

    impl Limited {
        fn new(data: Vec<u8>, limit: usize) -> Self {
            Self {
                inner: Inner { data },
                limit,
            }
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            let bytes = self.inner.chunk_mut();
            let end = std::cmp::min(bytes.len(), self.limit);
            &mut bytes[..end]
        }
    }

    let mut limited = Limited::new(vec![1, 2, 3, 4, 5], 3);
    let chunk = limited.chunk_mut();
    assert_eq!(chunk, &[1, 2, 3]);
}

#[test]
fn test_chunk_mut_exceeding_limit() {
    struct Inner {
        data: Vec<u8>,
    }

    impl Inner {
        fn chunk_mut(&mut self) -> &mut [u8] {
            &mut self.data
        }
    }

    struct Limited {
        inner: Inner,
        limit: usize,
    }

    impl Limited {
        fn new(data: Vec<u8>, limit: usize) -> Self {
            Self {
                inner: Inner { data },
                limit,
            }
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            let bytes = self.inner.chunk_mut();
            let end = std::cmp::min(bytes.len(), self.limit);
            &mut bytes[..end]
        }
    }

    let mut limited = Limited::new(vec![1, 2, 3], 5);
    let chunk = limited.chunk_mut();
    assert_eq!(chunk, &[1, 2, 3]);
}

#[test]
fn test_chunk_mut_empty() {
    struct Inner {
        data: Vec<u8>,
    }

    impl Inner {
        fn chunk_mut(&mut self) -> &mut [u8] {
            &mut self.data
        }
    }

    struct Limited {
        inner: Inner,
        limit: usize,
    }

    impl Limited {
        fn new(data: Vec<u8>, limit: usize) -> Self {
            Self {
                inner: Inner { data },
                limit,
            }
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            let bytes = self.inner.chunk_mut();
            let end = std::cmp::min(bytes.len(), self.limit);
            &mut bytes[..end]
        }
    }

    let mut limited = Limited::new(vec![], 3);
    let chunk = limited.chunk_mut();
    assert_eq!(chunk, &[]);
}

