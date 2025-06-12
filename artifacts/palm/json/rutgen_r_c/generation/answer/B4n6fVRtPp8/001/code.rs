// Answer 0

#[test]
fn test_serialize_i8_valid_input() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }
        
        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            self.write(buf)?;
            Ok(())
        }
        
        fn flush(&mut self) -> Result<()> {
            // No-op for mock
            Ok(())
        }
    }
    
    struct MockFormatter;
    
    impl MockFormatter {
        fn write_i8(&mut self, writer: &mut MockWriter, value: i8) -> Result<()> {
            writer.write(&[value as u8])?;
            Ok(())
        }
    }
    
    struct MockSerializer<'a> {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    impl<'a> ser::Serializer for &'a mut MockSerializer<'a> {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();

        fn serialize_i8(self, value: i8) -> Result<()> {
            self.formatter.write_i8(&mut self.writer, value).map_err(Error::io)
        }
        
        // Other methods are omitted for brevity
    }

    let mut writer = MockWriter { buffer: Vec::new() };
    let mut formatter = MockFormatter;
    let mut serializer = MockSerializer { writer, formatter };

    // Test with a normal value
    let result = serializer.serialize_i8(127);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.buffer, vec![127]);

    // Test with a negative value
    serializer.writer.buffer.clear(); // Clear buffer before next test
    let result = serializer.serialize_i8(-128);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.buffer, vec![128]); // -128 is represented as 128 in unsigned byte
}

