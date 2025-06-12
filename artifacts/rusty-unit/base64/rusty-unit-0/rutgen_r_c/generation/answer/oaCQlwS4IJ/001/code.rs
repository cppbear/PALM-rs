// Answer 0

#[test]
fn test_write_valid_utf8() {
    struct MockStrConsumer {
        received: String,
    }

    impl StrConsumer for MockStrConsumer {
        fn consume(&mut self, buf: &str) {
            self.received.push_str(buf);
        }
    }

    let mut consumer = MockStrConsumer { received: String::new() };
    let mut writer = Utf8SingleCodeUnitWriter { str_consumer: consumer };

    let input = "Hello, World!".as_bytes(); // valid UTF-8
    let result = writer.write(input).unwrap();

    assert_eq!(result, input.len());
    assert_eq!(writer.str_consumer.received, "Hello, World!");
}

#[test]
#[should_panic(expected = "Input must be valid UTF-8")]
fn test_write_invalid_utf8() {
    struct MockStrConsumer;

    impl StrConsumer for MockStrConsumer {
        fn consume(&mut self, _: &str) {
            // No operation, since this will panic before reaching here
        }
    }

    let mut consumer = MockStrConsumer;
    let mut writer = Utf8SingleCodeUnitWriter { str_consumer: consumer };

    let invalid_utf8: &[u8] = &[0, 159, 146, 150]; // invalid UTF-8 sequence
    writer.write(invalid_utf8).unwrap(); // This line should panic
}

