// Answer 0

fn test_peek_none_with_io_error() {
    struct TestIter {
        calls: usize,
    }

    impl Iterator for TestIter {
        type Item = Result<u8, std::io::Error>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.calls == 0 {
                self.calls += 1;
                Some(Err(std::io::Error::new(std::io::ErrorKind::Other, "IO error")))
            } else {
                None
            }
        }
    }

    struct TestStruct {
        ch: Option<u8>,
        iter: TestIter,
    }

    impl TestStruct {
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

    let mut test_struct = TestStruct {
        ch: None,
        iter: TestIter { calls: 0 },
    };
    
    let result = test_struct.peek();
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind(), std::io::ErrorKind::Other);
    }
}

