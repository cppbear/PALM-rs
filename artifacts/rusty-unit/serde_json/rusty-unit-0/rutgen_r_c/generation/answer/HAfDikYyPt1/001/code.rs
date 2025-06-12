// Answer 0

#[test]
fn test_serialize_unit_struct() {
    struct MockWriter {
        output: Vec<u8>,
        write_error: bool,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            if self.write_error {
                Err(Error::custom("Write error"))
            } else {
                self.output.extend_from_slice(buf);
                Ok(buf.len())
            }
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
        fn begin_array(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn end_array(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn begin_object(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn end_object(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn begin_object_key(&mut self, _: &mut dyn io::Write, _: bool) -> Result<()> {
            Ok(())
        }

        fn end_object_key(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn begin_object_value(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn begin_object_key_value(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut output = Vec::new();
    let mut writer = MockWriter { output, write_error: false };
    let formatter = MockFormatter;

    let mut serializer = Serializer {
        writer: &mut writer,
        formatter,
    };

    // Test successful serialization of unit struct
    let result = serializer.serialize_unit_struct("test_unit");
    assert!(result.is_ok());
    
    // Test write error in serialization
    writer.write_error = true;
    let result_error = serializer.serialize_unit_struct("test_unit");
    assert!(result_error.is_err());
}

