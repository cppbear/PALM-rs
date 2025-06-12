// Answer 0

fn peek_tests() {
    struct MockIter {
        values: Vec<Result<u8, std::io::Error>>,
        index: usize,
    }

    impl MockIter {
        fn new(values: Vec<Result<u8, std::io::Error>>) -> Self {
            Self { values, index: 0 }
        }
        
        fn next(&mut self) -> Option<Result<u8, std::io::Error>> {
            if self.index < self.values.len() {
                let result = self.values[self.index].clone();
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    struct MockReader {
        ch: Option<u8>,
        iter: MockIter,
    }

    impl MockReader {
        fn new(iter: MockIter) -> Self {
            Self { ch: None, iter }
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

    #[test]
    fn test_peek_with_none_and_success() {
        let mut reader = MockReader::new(MockIter::new(vec![Ok(42)]));
        assert_eq!(reader.peek().unwrap(), Ok(Some(42)));
    }

    #[test]
    fn test_peek_with_none_and_error() {
        let err = std::io::Error::new(std::io::ErrorKind::Other, "an error occurred");
        let mut reader = MockReader::new(MockIter::new(vec![Err(err)]));
        assert!(reader.peek().is_err());
    }

    #[test]
    fn test_peek_with_none_and_no_more() {
        let mut reader = MockReader::new(MockIter::new(vec![]));
        assert_eq!(reader.peek().unwrap(), Ok(None));
    }

    #[test]
    fn test_peek_with_existing_char() {
        let mut reader = MockReader {
            ch: Some(42),
            iter: MockIter::new(vec![]),
        };
        assert_eq!(reader.peek().unwrap(), Ok(Some(42)));
    }
}

