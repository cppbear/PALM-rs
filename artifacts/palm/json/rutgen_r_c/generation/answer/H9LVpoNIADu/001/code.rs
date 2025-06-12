// Answer 0

#[test]
fn test_discard_with_some_char() {
    struct TestReader {
        ch: Option<u8>,
    }

    impl TestReader {
        fn new(ch: Option<u8>) -> Self {
            Self { ch }
        }

        fn discard(&mut self) {
            self.ch = None;
        }
    }

    let mut reader = TestReader::new(Some(65)); // ASCII for 'A'
    reader.discard();
    assert_eq!(reader.ch, None);
}

#[test]
fn test_discard_with_none_char() {
    struct TestReader {
        ch: Option<u8>,
    }

    impl TestReader {
        fn new(ch: Option<u8>) -> Self {
            Self { ch }
        }

        fn discard(&mut self) {
            self.ch = None;
        }
    }

    let mut reader = TestReader::new(None);
    reader.discard();
    assert_eq!(reader.ch, None);
}

