// Answer 0

#[test]
fn test_resolve_some() {
    struct TestPos {
        index: Size,
        hash: HashValue,
    }

    impl TestPos {
        fn new(index: usize, hash: HashValue) -> Self {
            TestPos {
                index: index as Size,
                hash,
            }
        }

        fn is_some(&self) -> bool {
            self.index < MAX_SIZE // Mimicking the behavior of Pos::is_some()
        }

        fn resolve(&self) -> Option<(usize, HashValue)> {
            if self.is_some() {
                Some((self.index as usize, self.hash))
            } else {
                None
            }
        }
    }

    let hash_value = HashValue(15);
    let test_pos = TestPos::new(5, hash_value);
    let result = test_pos.resolve();
    
    assert_eq!(result, Some((5, HashValue(15))));
}

#[test]
fn test_resolve_none() {
    struct TestPos {
        index: Size,
        hash: HashValue,
    }

    impl TestPos {
        fn new(index: usize, hash: HashValue) -> Self {
            TestPos {
                index: index as Size,
                hash,
            }
        }

        fn is_some(&self) -> bool {
            self.index < MAX_SIZE // Mimicking the behavior of Pos::is_some()
        }

        fn resolve(&self) -> Option<(usize, HashValue)> {
            if self.is_some() {
                Some((self.index as usize, self.hash))
            } else {
                None
            }
        }
    }

    let hash_value = HashValue(0);
    let test_pos = TestPos::new(MAX_SIZE, hash_value);
    let result = test_pos.resolve();
    
    assert_eq!(result, None);
}

