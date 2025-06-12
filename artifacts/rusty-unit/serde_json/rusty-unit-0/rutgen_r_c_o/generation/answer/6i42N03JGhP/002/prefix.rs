// Answer 0

#[test]
fn test_serialize_u128_success() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_u128(&mut self, _writer: &mut dyn io::Write, _value: u128) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };

    let _ = key_serializer.serialize_u128(340282366920938463463374607431768211455);
}

#[test]
#[should_panic]
fn test_serialize_u128_write_fail() {
    struct MockWriterFail;
    impl io::Write for MockWriterFail {
        fn write(&mut self, _buf: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "fail"))
        }
        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatterSuccess;

    impl Formatter for MockFormatterSuccess {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_u128(&mut self, _writer: &mut dyn io::Write, _value: u128) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriterFail;
    let formatter = MockFormatterSuccess;
    let serializer = Serializer { writer, formatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };

    let _ = key_serializer.serialize_u128(12345);
}

#[test]
#[should_panic]
fn test_serialize_u128_begin_string_fail() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatterFail;

    impl Formatter for MockFormatterFail {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Err(Error::from("fail"))
        }

        fn write_u128(&mut self, _writer: &mut dyn io::Write, _value: u128) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatterFail;
    let serializer = Serializer { writer, formatter };
    let key_serializer = MapKeySerializer { ser: &mut serializer };

    let _ = key_serializer.serialize_u128(123456789);
}

