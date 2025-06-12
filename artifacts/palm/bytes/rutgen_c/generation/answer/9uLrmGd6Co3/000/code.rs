// Answer 0

#[test]
fn test_get_ref() {
    struct MockBuf {
        data: Vec<u8>,
    }

    impl MockBuf {
        fn remaining(&self) -> usize {
            self.data.len()
        }
    }

    let mock_buf = MockBuf {
        data: vec![b'a', b'b', b'c'],
    };
    
    let iter = IntoIter::new(mock_buf);
    
    assert_eq!(iter.get_ref().remaining(), 3);
}

#[test]
fn test_get_ref_empty() {
    struct MockBuf {
        data: Vec<u8>,
    }

    impl MockBuf {
        fn remaining(&self) -> usize {
            self.data.len()
        }
    }

    let mock_buf = MockBuf {
        data: vec![],
    };
    
    let iter = IntoIter::new(mock_buf);
    
    assert_eq!(iter.get_ref().remaining(), 0);
}

