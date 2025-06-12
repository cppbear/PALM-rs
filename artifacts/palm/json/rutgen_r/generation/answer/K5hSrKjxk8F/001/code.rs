// Answer 0

#[test]
fn test_next_char_some_value() {
    struct TestReader {
        data: Vec<u8>,
        index: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            TestReader { data, index: 0 }
        }

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                let value = self.data[self.index];
                self.index += 1;
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }
    }

    let mut reader = TestReader::new(vec![104, 101, 108, 108, 111]); // "hello" in UTF-8
    assert_eq!(reader.next(), Ok(Some(104))); // 'h'
    assert_eq!(reader.next(), Ok(Some(101))); // 'e'
    assert_eq!(reader.next(), Ok(Some(108))); // 'l'
    assert_eq!(reader.next(), Ok(Some(108))); // 'l'
    assert_eq!(reader.next(), Ok(Some(111))); // 'o'
    assert_eq!(reader.next(), Ok(None)); // End of data
}

#[test]
fn test_next_char_empty() {
    struct TestReader {
        data: Vec<u8>,
        index: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            TestReader { data, index: 0 }
        }

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                let value = self.data[self.index];
                self.index += 1;
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }
    }

    let mut reader = TestReader::new(vec![]);
    assert_eq!(reader.next(), Ok(None)); // No data
}

#[should_panic]
#[test]
fn test_next_char_panic() {
    struct TestReader {
        data: Vec<u8>,
        index: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            TestReader { data, index: 0 }
        }

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                let value = self.data[self.index];
                self.index += 1;
                Ok(Some(value))
            } else {
                panic!("Attempted to read past end of data");
            }
        }
    }

    let mut reader = TestReader::new(vec![97, 98, 99]); // "abc" in UTF-8
    for _ in 0..4 {
        reader.next(); // This will panic on the fourth call
    }
}

#[test]
fn test_next_char_large_input() {
    struct TestReader {
        data: Vec<u8>,
        index: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            TestReader { data, index: 0 }
        }

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                let value = self.data[self.index];
                self.index += 1;
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }
    }

    let data = (0..256).collect::<Vec<u8>>(); // Large input from 0 to 255
    let mut reader = TestReader::new(data);
    
    for i in 0..256 {
        assert_eq!(reader.next(), Ok(Some(i as u8)));
    }
    assert_eq!(reader.next(), Ok(None)); // End of data
}

