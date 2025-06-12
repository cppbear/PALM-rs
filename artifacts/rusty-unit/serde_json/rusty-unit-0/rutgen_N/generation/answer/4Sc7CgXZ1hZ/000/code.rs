// Answer 0

#[test]
fn test_peek_some_value() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }

        fn peek(&mut self) -> Result<Option<u8>, std::io::Error> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }
    }

    struct TestStruct {
        read: MockReader,
    }

    impl TestStruct {
        pub(crate) fn new(data: Vec<u8>) -> Self {
            Self { read: MockReader::new(data) }
        }

        pub(crate) fn peek(&mut self) -> Result<Option<u8>, std::io::Error> {
            self.read.peek()
        }
    }

    let mut test_struct = TestStruct::new(vec![1, 2, 3]);
    let result = test_struct.peek().unwrap();
    assert_eq!(result, Some(1));
}

#[test]
fn test_peek_none_value() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }

        fn peek(&mut self) -> Result<Option<u8>, std::io::Error> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }
    }

    struct TestStruct {
        read: MockReader,
    }

    impl TestStruct {
        pub(crate) fn new(data: Vec<u8>) -> Self {
            Self { read: MockReader::new(data) }
        }

        pub(crate) fn peek(&mut self) -> Result<Option<u8>, std::io::Error> {
            self.read.peek()
        }
    }

    let mut test_struct = TestStruct::new(vec![]);
    let result = test_struct.peek().unwrap();
    assert_eq!(result, None);
}

