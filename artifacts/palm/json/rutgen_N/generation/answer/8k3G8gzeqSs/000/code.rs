// Answer 0

#[test]
fn test_peek_some_char() {
    struct TestIterator {
        chars: Vec<u8>,
        index: usize,
    }

    impl TestIterator {
        fn new(chars: Vec<u8>) -> Self {
            TestIterator { chars, index: 0 }
        }
    }

    impl Iterator for TestIterator {
        type Item = Result<u8, std::io::Error>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.chars.len() {
                let ch = self.chars[self.index];
                self.index += 1;
                Some(Ok(ch))
            } else {
                None
            }
        }
    }

    struct TestPeek {
        iter: TestIterator,
        ch: Option<u8>,
    }

    impl TestPeek {
        fn new(chars: Vec<u8>) -> Self {
            TestPeek {
                iter: TestIterator::new(chars),
                ch: None,
            }
        }
        
        fn peek(&mut self) -> Result<Option<u8>, std::io::Error> {
            match self.ch {
                Some(ch) => Ok(Some(ch)),
                None => match self.iter.next() {
                    Some(Err(err)) => Err(err),
                    Some(Ok(ch)) => {
                        self.ch = Some(ch);
                        Ok(self.ch)
                    }
                    None => Ok(None),
                },
            }
        }
    }

    let mut test_peek = TestPeek::new(vec![97, 98, 99]); // 'a', 'b', 'c'
    let result = test_peek.peek().unwrap();
    assert_eq!(result, Some(97));
}

#[test]
fn test_peek_none_char() {
    struct TestIterator {
        chars: Vec<u8>,
        index: usize,
    }

    impl TestIterator {
        fn new(chars: Vec<u8>) -> Self {
            TestIterator { chars, index: 0 }
        }
    }

    impl Iterator for TestIterator {
        type Item = Result<u8, std::io::Error>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.chars.len() {
                let ch = self.chars[self.index];
                self.index += 1;
                Some(Ok(ch))
            } else {
                None
            }
        }
    }

    struct TestPeek {
        iter: TestIterator,
        ch: Option<u8>,
    }

    impl TestPeek {
        fn new(chars: Vec<u8>) -> Self {
            TestPeek {
                iter: TestIterator::new(chars),
                ch: None,
            }
        }
        
        fn peek(&mut self) -> Result<Option<u8>, std::io::Error> {
            match self.ch {
                Some(ch) => Ok(Some(ch)),
                None => match self.iter.next() {
                    Some(Err(err)) => Err(err),
                    Some(Ok(ch)) => {
                        self.ch = Some(ch);
                        Ok(self.ch)
                    }
                    None => Ok(None),
                },
            }
        }
    }

    let mut test_peek = TestPeek::new(vec![]);
    let result = test_peek.peek().unwrap();
    assert_eq!(result, None);
}

