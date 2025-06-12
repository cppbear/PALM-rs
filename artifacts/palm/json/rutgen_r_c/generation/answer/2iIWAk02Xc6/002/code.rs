// Answer 0

#[test]
fn test_serialize_seq_empty_array() {
    struct MockFormatter;

    impl MockFormatter {
        fn begin_array(&mut self) -> Result<()> {
            Ok(())
        }
        fn end_array(&mut self) -> Result<()> {
            Err(Error)
        }
    }

    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn write_all(&mut self, _buf: &[u8]) -> Result<()> {
            Ok(())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut formatter = MockFormatter;
    let writer = MockWriter;
    let serializer = Serializer {
        writer,
        formatter,
    };

    let result = serializer.serialize_seq(Some(0));
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_serialize_seq_non_empty_array() {
    struct MockFormatter;

    impl MockFormatter {
        fn begin_array(&mut self) -> Result<()> {
            Ok(())
        }
        fn end_array(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn write_all(&mut self, _buf: &[u8]) -> Result<()> {
            Ok(())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut formatter = MockFormatter;
    let writer = MockWriter;
    let serializer = Serializer {
        writer,
        formatter,
    };

    let result = serializer.serialize_seq(Some(1));
    assert!(result.is_ok());
}

