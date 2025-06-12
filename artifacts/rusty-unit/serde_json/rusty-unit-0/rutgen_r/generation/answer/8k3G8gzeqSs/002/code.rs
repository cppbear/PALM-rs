// Answer 0

fn test_peek_none_with_error() {
    struct TestIter {
        values: Vec<Result<u8, std::io::Error>>,
        index: usize,
    }

    impl TestIter {
        fn new(values: Vec<Result<u8, std::io::Error>>) -> Self {
            TestIter { values, index: 0 }
        }
    }

    impl Iterator for TestIter {
        type Item = Result<u8, std::io::Error>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.values.len() {
                let result = self.values[self.index].clone();
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    struct Peekable {
        ch: Option<u8>,
        iter: TestIter,
    }

    impl Peekable {
        fn new(iter: TestIter) -> Self {
            Peekable { ch: None, iter }
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

    let iter = TestIter::new(vec![Err(std::io::Error::new(std::io::ErrorKind::Other, "an error"))]);
    let mut peekable = Peekable::new(iter);
    
    let result = peekable.peek();
    
    assert!(result.is_err());
}

fn test_peek_some_ok() {
    struct TestIter {
        values: Vec<Result<u8, std::io::Error>>,
        index: usize,
    }

    impl TestIter {
        fn new(values: Vec<Result<u8, std::io::Error>>) -> Self {
            TestIter { values, index: 0 }
        }
    }

    impl Iterator for TestIter {
        type Item = Result<u8, std::io::Error>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.values.len() {
                let result = self.values[self.index].clone();
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    struct Peekable {
        ch: Option<u8>,
        iter: TestIter,
    }

    impl Peekable {
        fn new(iter: TestIter) -> Self {
            Peekable { ch: None, iter }
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

    let iter = TestIter::new(vec![Ok(42)]);
    let mut peekable = Peekable::new(iter);
    
    peekable.peek().unwrap(); // Should set self.ch to Some(42)
    let result = peekable.peek();
    
    assert_eq!(result, Ok(Some(42)));
}

fn test_peek_none() {
    struct TestIter {
        values: Vec<Result<u8, std::io::Error>>,
        index: usize,
    }

    impl TestIter {
        fn new(values: Vec<Result<u8, std::io::Error>>) -> Self {
            TestIter { values, index: 0 }
        }
    }

    impl Iterator for TestIter {
        type Item = Result<u8, std::io::Error>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.values.len() {
                let result = self.values[self.index].clone();
                self.index += 1;
                Some(result)
            } else {
                None
            }
        }
    }

    struct Peekable {
        ch: Option<u8>,
        iter: TestIter,
    }

    impl Peekable {
        fn new(iter: TestIter) -> Self {
            Peekable { ch: None, iter }
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

    let iter = TestIter::new(vec![]);
    let mut peekable = Peekable::new(iter);
    
    let result = peekable.peek();
    
    assert_eq!(result, Ok(None));
}

