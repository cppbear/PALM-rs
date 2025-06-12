// Answer 0

#[test]
fn test_next_char_or_null_returns_next_char() {
    struct TestIterator {
        chars: Vec<u8>,
        index: usize,
    }

    impl TestIterator {
        fn new(chars: Vec<u8>) -> Self {
            TestIterator { chars, index: 0 }
        }

        fn next_char(&mut self) -> Option<Result<u8, ()>> {
            if self.index < self.chars.len() {
                let result = Ok(self.chars[self.index]);
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    let mut iter = TestIterator::new(vec![b'a', b'b', b'c']);
    assert_eq!(iter.next_char_or_null().unwrap(), b'a');
    assert_eq!(iter.next_char_or_null().unwrap(), b'b');
    assert_eq!(iter.next_char_or_null().unwrap(), b'c');
    assert_eq!(iter.next_char_or_null().unwrap(), b'\x00');
}

#[test]
fn test_next_char_or_null_returns_null() {
    struct TestIterator {
        chars: Vec<u8>,
        index: usize,
    }

    impl TestIterator {
        fn new(chars: Vec<u8>) -> Self {
            TestIterator { chars, index: 0 }
        }

        fn next_char(&mut self) -> Option<Result<u8, ()>> {
            None
        }
    }

    let mut iter = TestIterator::new(vec![]);
    assert_eq!(iter.next_char_or_null().unwrap(), b'\x00');
}

