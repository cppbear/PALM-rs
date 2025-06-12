// Answer 0

#[test]
fn test_serialize_map_empty() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        
        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            self.write(buf)?;
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_object(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn end_object(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    let result = serializer.serialize_map(Some(0)).unwrap();

    match result {
        Compound::Map { ser, state } => {
            // Check if serializer state is empty
            assert_eq!(state, State::Empty);
            // Assert that no additional state is maintained
            assert_eq!(ser.writer.output.len(), 0); // Verify no write happened
        }
    }
}

