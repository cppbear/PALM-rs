// Answer 0

#[test]
fn test_chunk_with_remaining_a() {
    struct MockA {
        remaining: usize,
        data: Vec<u8>,
    }

    impl MockA {
        fn has_remaining(&self) -> bool {
            self.remaining > 0
        }

        fn chunk(&self) -> &[u8] {
            &self.data
        }
    }

    struct MockB {
        data: Vec<u8>,
    }

    impl MockB {
        fn chunk(&self) -> &[u8] {
            &self.data
        }
    }

    struct Chain {
        a: MockA,
        b: MockB,
    }

    impl Chain {
        fn chunk(&self) -> &[u8] {
            if self.a.has_remaining() {
                self.a.chunk()
            } else {
                self.b.chunk()
            }
        }
    }

    let a = MockA {
        remaining: 1,
        data: vec![1, 2, 3],
    };
    let b = MockB {
        data: vec![4, 5, 6],
    };
    let chain = Chain { a, b };

    assert_eq!(chain.chunk(), &[1, 2, 3]);
}

#[test]
fn test_chunk_without_remaining_a() {
    struct MockA {
        remaining: usize,
        data: Vec<u8>,
    }

    impl MockA {
        fn has_remaining(&self) -> bool {
            self.remaining > 0
        }

        fn chunk(&self) -> &[u8] {
            &self.data
        }
    }

    struct MockB {
        data: Vec<u8>,
    }

    impl MockB {
        fn chunk(&self) -> &[u8] {
            &self.data
        }
    }

    struct Chain {
        a: MockA,
        b: MockB,
    }

    impl Chain {
        fn chunk(&self) -> &[u8] {
            if self.a.has_remaining() {
                self.a.chunk()
            } else {
                self.b.chunk()
            }
        }
    }

    let a = MockA {
        remaining: 0,
        data: vec![1, 2, 3],
    };
    let b = MockB {
        data: vec![4, 5, 6],
    };
    let chain = Chain { a, b };

    assert_eq!(chain.chunk(), &[4, 5, 6]);
}

