// Answer 0

fn test_peek_end_of_value() {
    struct MockRead {
        return_value: Result<Option<u8>>,
    }

    impl read::Read<'_> for MockRead {
        fn peek(&mut self) -> Result<Option<u8>> {
            self.return_value.clone()
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: 1 }
        }
        
        fn byte_offset(&self) -> usize {
            0
        }
    }

    struct MockDeserializer {
        read: MockRead,
    }

    impl<'de> Deserializer<MockRead> {
        fn new(read: MockRead) -> Self {
            Deserializer {
                read,
                scratch: Vec::new(),
                remaining_depth: 0,
                #[cfg(feature = "float_roundtrip")]
                single_precision: false,
                #[cfg(feature = "unbounded_depth")]
                disable_recursion_limit: false,
            }
        }
    }

    struct MockStreamDeserializer {
        de: Deserializer<MockRead>,
    }

    let mut deserializer = MockStreamDeserializer {
        de: MockDeserializer::new(MockRead { return_value: Ok(Some(b' '))}),
    };
    assert_eq!(deserializer.peek_end_of_value(), Ok(()));

    deserializer.de = MockDeserializer::new(MockRead { return_value: Ok(Some(b'x'))});
    assert_eq!(deserializer.peek_end_of_value(),
               Err(Error::syntax(ErrorCode::TrailingCharacters, 1, 1)));

    deserializer.de = MockDeserializer::new(MockRead { return_value: Ok(None)});
    assert_eq!(deserializer.peek_end_of_value(), Ok(()));

    deserializer.de = MockDeserializer::new(MockRead { return_value: Err(ErrorCode::Io(std::io::Error::new(std::io::ErrorKind::Other, "error"))) });
    assert_eq!(deserializer.peek_end_of_value(),
               Err(Error::syntax(ErrorCode::TrailingCharacters, 1, 1)));
}

