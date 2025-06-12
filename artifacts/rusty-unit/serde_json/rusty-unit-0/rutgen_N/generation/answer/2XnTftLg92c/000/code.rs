// Answer 0

#[test]
fn test_serialize_u64_success() {
    struct MockFormatter {
        written: String,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _: &mut dyn std::io::Write) -> std::io::Result<()> {
            self.written.push('"');
            Ok(())
        }

        fn write_u64(&mut self, _: &mut dyn std::io::Write, value: u64) -> std::io::Result<()> {
            self.written += &value.to_string();
            Ok(())
        }

        fn end_string(&mut self, _: &mut dyn std::io::Write) -> std::io::Result<()> {
            self.written.push('"');
            Ok(())
        }
    }

    struct MockWriter {
        output: Vec<u8>,
    }

    impl std::io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct Serializer {
        formatter: MockFormatter,
        writer: MockWriter,
    }

    let writer = MockWriter { output: vec![] };
    let formatter = MockFormatter { written: String::new() };
    let mut ser = Serializer { formatter, writer };

    let result = ser.serialize_u64(42);
    assert!(result.is_ok());
    assert_eq!(ser.formatter.written, r#""""42""#);
}

#[test]
#[should_panic]
fn test_serialize_u64_io_failure() {
    struct FailingFormatter;

    impl FailingFormatter {
        fn begin_string(&mut self, _: &mut dyn std::io::Write) -> std::io::Result<()> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "IO failure"))
        }

        fn write_u64(&mut self, _: &mut dyn std::io::Write, _: u64) -> std::io::Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut dyn std::io::Write) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct MockWriter {
        output: Vec<u8>,
    }

    impl std::io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct Serializer {
        formatter: FailingFormatter,
        writer: MockWriter,
    }

    let writer = MockWriter { output: vec![] };
    let formatter = FailingFormatter {};
    let mut ser = Serializer { formatter, writer };

    ser.serialize_u64(42).unwrap(); // This will panic due to IO failure.
}

