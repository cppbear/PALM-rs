// Answer 0

#[test]
fn test_stream_deserializer_new() {
    struct MockRead {
        offset: usize,
    }

    impl read::Read<'static> for MockRead {
        fn byte_offset(&self) -> usize {
            self.offset
        }
    }

    let mock_read = MockRead { offset: 10 };
    let deserializer: StreamDeserializer<_, ()> = StreamDeserializer::new(mock_read);
    assert_eq!(deserializer.offset, 10);
}

#[test]
fn test_stream_deserializer_new_with_different_offset() {
    struct AnotherMockRead {
        offset: usize,
    }

    impl read::Read<'static> for AnotherMockRead {
        fn byte_offset(&self) -> usize {
            self.offset
        }
    }

    let another_mock_read = AnotherMockRead { offset: 0 };
    let deserializer: StreamDeserializer<_, ()> = StreamDeserializer::new(another_mock_read);
    assert_eq!(deserializer.offset, 0);
}

#[test]
fn test_stream_deserializer_new_with_different_read_values() {
    struct FinalMockRead {
        offset: usize,
    }

    impl read::Read<'static> for FinalMockRead {
        fn byte_offset(&self) -> usize {
            self.offset
        }
    }

    let final_mock_read = FinalMockRead { offset: 1234 };
    let deserializer: StreamDeserializer<_, ()> = StreamDeserializer::new(final_mock_read);
    assert_eq!(deserializer.offset, 1234);
}

