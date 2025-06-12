// Answer 0

#[test]
fn test_chunk_within_limits() {
    struct Inner {
        data: Vec<u8>,
    }

    impl Inner {
        fn chunk(&self) -> &[u8] {
            &self.data
        }
    }

    struct Chunker<'a> {
        inner: &'a Inner,
        limit: usize,
    }

    let inner = Inner { data: vec![1, 2, 3, 4, 5] };
    let chunker = Chunker { inner: &inner, limit: 3 };

    let result = chunker.chunk();
    assert_eq!(result, &[1, 2, 3]);
}

#[test]
fn test_chunk_exceeds_limits() {
    struct Inner {
        data: Vec<u8>,
    }

    impl Inner {
        fn chunk(&self) -> &[u8] {
            &self.data
        }
    }

    struct Chunker<'a> {
        inner: &'a Inner,
        limit: usize,
    }

    let inner = Inner { data: vec![1, 2, 3, 4, 5] };
    let chunker = Chunker { inner: &inner, limit: 10 };

    let result = chunker.chunk();
    assert_eq!(result, &[1, 2, 3, 4, 5]);
}

#[test]
fn test_chunk_empty() {
    struct Inner {
        data: Vec<u8>,
    }

    impl Inner {
        fn chunk(&self) -> &[u8] {
            &self.data
        }
    }

    struct Chunker<'a> {
        inner: &'a Inner,
        limit: usize,
    }

    let inner = Inner { data: vec![] };
    let chunker = Chunker { inner: &inner, limit: 5 };

    let result = chunker.chunk();
    assert_eq!(result, &[]);
}

