// Answer 0

#[test]
fn test_chunk_when_a_has_remaining() {
    struct MockA {
        data: Vec<u8>,
        position: usize,
    }
    
    impl MockA {
        fn has_remaining(&self) -> bool {
            self.position < self.data.len()
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
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

    struct Chain<'a> {
        a: &'a MockA,
        b: &'a MockB,
    }

    let a = MockA {
        data: vec![1, 2, 3, 4],
        position: 0,
    };

    let b = MockB {
        data: vec![5, 6, 7, 8],
    };

    let chain = Chain { a: &a, b: &b };

    let result = chain.chunk();
    assert_eq!(result, &vec![1, 2, 3, 4][0..]);
}

#[test]
fn test_chunk_when_a_empty() {
    struct MockA {
        data: Vec<u8>,
        position: usize,
    }

    impl MockA {
        fn has_remaining(&self) -> bool {
            self.position < self.data.len()
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
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

    struct Chain<'a> {
        a: &'a MockA,
        b: &'a MockB,
    }

    let a = MockA {
        data: vec![],
        position: 0,
    };

    let b = MockB {
        data: vec![5, 6, 7, 8],
    };

    let chain = Chain { a: &a, b: &b };

    let result = chain.chunk();
    assert_eq!(result, &vec![5, 6, 7, 8][..]);
}

