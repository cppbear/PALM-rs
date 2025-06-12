// Answer 0

#[test]
fn test_serialize_str_empty() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {}

    let mut writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };

    assert!(serializer.serialize_str("").is_ok());
}

#[test]
fn test_serialize_str_escape() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {}

    let mut writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };

    assert!(serializer.serialize_str("Hello, \"World!\"").is_ok());
}

#[test]
fn test_serialize_str_special_characters() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {}

    let mut writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };

    assert!(serializer.serialize_str("Line1\nLine2\tTab").is_ok());
}

#[test]
#[should_panic]
fn test_serialize_str_overflow() {
    struct PanicWriter;

    impl io::Write for PanicWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            panic!("Write method panicked!");
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {}

    let writer = PanicWriter;
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };

    let _ = serializer.serialize_str("This should panic on write");
}

