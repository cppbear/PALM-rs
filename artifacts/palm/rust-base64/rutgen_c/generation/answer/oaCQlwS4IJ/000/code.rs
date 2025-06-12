// Answer 0

#[test]
fn test_write_valid_utf8() {
    struct TestConsumer {
        output: String,
    }

    impl StrConsumer for TestConsumer {
        fn consume(&mut self, buf: &str) {
            self.output.push_str(buf);
        }
    }

    let mut consumer = TestConsumer { output: String::new() };
    let mut writer = Utf8SingleCodeUnitWriter { str_consumer: consumer };

    let input = b"hello";
    let result = writer.write(input).unwrap();
    
    assert_eq!(result, input.len());
    assert_eq!(writer.str_consumer.output, "hello");
}

#[test]
#[should_panic(expected = "Input must be valid UTF-8")]
fn test_write_invalid_utf8() {
    struct TestConsumer;

    impl StrConsumer for TestConsumer {
        fn consume(&mut self, _: &str) {}
    }

    let mut consumer = TestConsumer;
    let mut writer = Utf8SingleCodeUnitWriter { str_consumer: consumer };

    let input: &[u8] = &[0xff];
    writer.write(input).unwrap();
}

#[test]
fn test_write_empty() {
    struct TestConsumer {
        output: String,
    }

    impl StrConsumer for TestConsumer {
        fn consume(&mut self, buf: &str) {
            self.output.push_str(buf);
        }
    }

    let mut consumer = TestConsumer { output: String::new() };
    let mut writer = Utf8SingleCodeUnitWriter { str_consumer: consumer };

    let input: &[u8] = b"";
    let result = writer.write(input).unwrap();
    
    assert_eq!(result, 0);
    assert_eq!(writer.str_consumer.output, "");
}

