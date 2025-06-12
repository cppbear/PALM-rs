// Answer 0

#[test]
fn test_write_valid_utf8() {
    struct MockConsumer {
        consumed: String,
    }

    impl MockConsumer {
        fn new() -> Self {
            MockConsumer {
                consumed: String::new(),
            }
        }

        fn consume(&mut self, s: &str) {
            self.consumed.push_str(s);
        }
    }

    struct EncoderStringWriter {
        str_consumer: MockConsumer,
    }

    impl EncoderStringWriter {
        fn new() -> Self {
            EncoderStringWriter {
                str_consumer: MockConsumer::new(),
            }
        }

        fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
            let s = std::str::from_utf8(buf).expect("Input must be valid UTF-8");
            self.str_consumer.consume(s);
            Ok(buf.len())
        }
    }

    let mut writer = EncoderStringWriter::new();
    let input = b"Hello, World!";
    let result = writer.write(input).unwrap();
    assert_eq!(result, input.len());
}

#[test]
#[should_panic(expected = "Input must be valid UTF-8")]
fn test_write_invalid_utf8() {
    struct MockConsumer {
        consumed: String,
    }

    impl MockConsumer {
        fn new() -> Self {
            MockConsumer {
                consumed: String::new(),
            }
        }

        fn consume(&mut self, _s: &str) {
            // Intentionally left blank for this test
        }
    }

    struct EncoderStringWriter {
        str_consumer: MockConsumer,
    }

    impl EncoderStringWriter {
        fn new() -> Self {
            EncoderStringWriter {
                str_consumer: MockConsumer::new(),
            }
        }

        fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
            let s = std::str::from_utf8(buf).expect("Input must be valid UTF-8");
            self.str_consumer.consume(s);
            Ok(buf.len())
        }
    }

    let mut writer = EncoderStringWriter::new();
    let invalid_input = &[0, 159, 146, 150]; // Invalid UTF-8 bytes
    writer.write(invalid_input);
}

