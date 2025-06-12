// Answer 0

#[test]
fn test_serialize_u64_success() {
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

    impl Formatter for MockFormatter {
        fn begin_string<W: io::Write>(&self, _writer: &mut W) -> Result<()> {
            Ok(())
        }

        fn write_u64<W: io::Write>(&self, _writer: &mut W, value: u64) -> Result<()> {
            // Simulate writing u64 by converting to string
            let str_value = value.to_string();
            Ok(())
        }

        fn end_string<W: io::Write>(&self, _writer: &mut W) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    let mut map_key_serializer = MapKeySerializer { ser: &mut serializer };

    assert!(map_key_serializer.serialize_u64(42).is_ok());
}

#[test]
#[should_panic]
fn test_serialize_u64_write_error() {
    struct MockWriter {
        should_fail: bool,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            if self.should_fail {
                Err(Error::new())
            } else {
                Ok(0)
            }
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string<W: io::Write>(&self, _writer: &mut W) -> Result<()> {
            Ok(())
        }

        fn write_u64<W: io::Write>(&self, _writer: &mut W, _value: u64) -> Result<()> {
            Err(Error::new())
        }

        fn end_string<W: io::Write>(&self, _writer: &mut W) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter { should_fail: true };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    let mut map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let _result = map_key_serializer.serialize_u64(42);
}

