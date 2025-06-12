// Answer 0

#[test]
fn test_serialize_i16_positive() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            assert_eq!(buf, &[0x00, 0x01]); // Expecting serialized value for 1
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
        fn write_i16(&self, writer: &mut impl io::Write, value: i16) -> Result<()> {
            writer.write(&value.to_be_bytes())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    // Test positive i16 value
    let result = serializer.serialize_i16(1);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_i16_negative() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            assert_eq!(buf, &[0xFF, 0xFE]); // Expecting serialized value for -2
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
        fn write_i16(&self, writer: &mut impl io::Write, value: i16) -> Result<()> {
            writer.write(&value.to_be_bytes())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    // Test negative i16 value
    let result = serializer.serialize_i16(-2);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_i16_zero() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            assert_eq!(buf, &[0x00, 0x00]); // Expecting serialized value for 0
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
        fn write_i16(&self, writer: &mut impl io::Write, value: i16) -> Result<()> {
            writer.write(&value.to_be_bytes())
        }
    }

    let writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    // Test zero i16 value
    let result = serializer.serialize_i16(0);
    assert!(result.is_ok());
}

