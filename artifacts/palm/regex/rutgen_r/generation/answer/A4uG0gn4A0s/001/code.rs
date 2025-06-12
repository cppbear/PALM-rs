// Answer 0

#[test]
fn test_approximate_size_empty_pattern() {
    struct TestPattern {
        pattern: Vec<u8>,
    }
    
    impl TestPattern {
        fn approximate_size(&self) -> usize {
            (self.pattern.len() * std::mem::size_of::<u8>())
                + (256 * std::mem::size_of::<usize>()) // skip table
        }
    }

    let empty_test_pattern = TestPattern {
        pattern: Vec::new(),
    };
    let expected_size = 256 * std::mem::size_of::<usize>();
    assert_eq!(empty_test_pattern.approximate_size(), expected_size);
}

#[test]
fn test_approximate_size_non_empty_pattern() {
    struct TestPattern {
        pattern: Vec<u8>,
    }
    
    impl TestPattern {
        fn approximate_size(&self) -> usize {
            (self.pattern.len() * std::mem::size_of::<u8>())
                + (256 * std::mem::size_of::<usize>()) // skip table
        }
    }

    let non_empty_test_pattern = TestPattern {
        pattern: vec![b'a', b'b', b'c'],
    };
    let expected_size = (3 * std::mem::size_of::<u8>()) + (256 * std::mem::size_of::<usize>());
    assert_eq!(non_empty_test_pattern.approximate_size(), expected_size);
}

#[test]
fn test_approximate_size_large_pattern() {
    struct TestPattern {
        pattern: Vec<u8>,
    }
    
    impl TestPattern {
        fn approximate_size(&self) -> usize {
            (self.pattern.len() * std::mem::size_of::<u8>())
                + (256 * std::mem::size_of::<usize>()) // skip table
        }
    }

    let large_test_pattern = TestPattern {
        pattern: vec![b'a'; 1000], // a pattern of 1000 bytes
    };
    let expected_size = (1000 * std::mem::size_of::<u8>()) + (256 * std::mem::size_of::<usize>());
    assert_eq!(large_test_pattern.approximate_size(), expected_size);
}

