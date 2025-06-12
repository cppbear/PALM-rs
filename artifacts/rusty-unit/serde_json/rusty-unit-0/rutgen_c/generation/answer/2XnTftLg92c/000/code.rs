// Answer 0

#[test]
fn test_serialize_u64() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_u64(&mut self, _writer: &mut dyn io::Write, value: u64) -> Result<()> {
            let serialized = value.to_string().into_bytes();
            _writer.write(&serialized).map(|_| ())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter {};
    let serializer = Serializer { writer, formatter };

    let mut map_key_serializer = MapKeySerializer { ser: &mut serializer };
    
    let result = map_key_serializer.serialize_u64(12345);
    
    assert!(result.is_ok());
    assert_eq!(map_key_serializer.ser.writer.output, b"12345");
}

