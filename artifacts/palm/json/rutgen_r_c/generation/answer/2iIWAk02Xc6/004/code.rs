// Answer 0

fn test_serialize_seq_non_empty() {
    struct MockWriter {
        written: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.written.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            self.written.extend_from_slice(buf);
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_array(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }

        fn end_array(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter { written: Vec::new() };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    // Test with non-zero length
    let result = serializer.serialize_seq(Some(5)).unwrap();
    match result {
        Compound::Map { ser, state } => {
            assert!(state == State::First);
            assert!(ser.writer.written.is_empty()); // No data should have been written yet
        }
    }
}

fn test_serialize_seq_empty() {
    struct MockWriter {
        written: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.written.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            self.written.extend_from_slice(buf);
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_array(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }

        fn end_array(&mut self, _writer: &mut impl io::Write) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter { written: Vec::new() };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    // Test with length of None (implying that it's treated as empty)
    let result = serializer.serialize_seq(None).unwrap();
    match result {
        Compound::Map { ser, state } => {
            assert!(state == State::First); // Ensure even zero-length still indicates state as First
            assert!(ser.writer.written.is_empty()); // No data should have been written yet
        }
    }
}

