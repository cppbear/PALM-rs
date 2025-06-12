// Answer 0

#[test]
fn test_new_with_str() {
    struct MockReader {
        offset: usize,
        data: &'static str,
    }

    impl MockReader {
        fn byte_offset(&self) -> usize {
            self.offset
        }
    }

    let reader = MockReader {
        offset: 0,
        data: r#"{"key": "value"}"#,
    };

    let deserializer = serde_json::StreamDeserializer::new(reader);
    assert!(!deserializer.failed);
}

#[test]
fn test_new_with_slice() {
    struct MockReaderSlice<'a> {
        offset: usize,
        data: &'a [u8],
    }

    impl<'a> MockReaderSlice<'a> {
        fn byte_offset(&self) -> usize {
            self.offset
        }
    }

    let reader_slice = MockReaderSlice {
        offset: 1,
        data: br#"{"key": "value"}"#,
    };

    let deserializer = serde_json::StreamDeserializer::new(reader_slice);
    assert_eq!(deserializer.offset, 1);
    assert!(!deserializer.failed);
}

#[test]
fn test_new_with_reader() {
    struct MockReaderIo {
        offset: usize,
        data: Vec<u8>,
    }

    impl MockReaderIo {
        fn byte_offset(&self) -> usize {
            self.offset
        }
    }

    let reader_io = MockReaderIo {
        offset: 2,
        data: br#"{"key": "value"}"#.to_vec(),
    };

    let deserializer = serde_json::StreamDeserializer::new(reader_io);
    assert_eq!(deserializer.offset, 2);
    assert!(!deserializer.failed);
}

