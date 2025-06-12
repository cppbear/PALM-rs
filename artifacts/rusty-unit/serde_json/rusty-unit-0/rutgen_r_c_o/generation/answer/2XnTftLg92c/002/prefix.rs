// Answer 0

#[test]
fn test_serialize_u64_with_ok_begin_string_and_err_write() {
    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct DummyFormatter;
    impl Formatter for DummyFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_u64(&mut self, _writer: &mut dyn io::Write, _value: u64) -> Result<()> {
            Err(Error)
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = DummyWriter;
    let mut formatter = DummyFormatter;
    let serializer = Serializer {
        writer: &mut writer,
        formatter: &mut formatter,
    };

    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let _ = map_key_serializer.serialize_u64(42);
}

#[test]
fn test_serialize_u64_with_ok_begin_string_and_err_write_zero() {
    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct DummyFormatter;
    impl Formatter for DummyFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_u64(&mut self, _writer: &mut dyn io::Write, _value: u64) -> Result<()> {
            Err(Error)
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = DummyWriter;
    let mut formatter = DummyFormatter;
    let serializer = Serializer {
        writer: &mut writer,
        formatter: &mut formatter,
    };

    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let _ = map_key_serializer.serialize_u64(0);
}

#[test]
fn test_serialize_u64_with_ok_begin_string_and_err_write_max() {
    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct DummyFormatter;
    impl Formatter for DummyFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_u64(&mut self, _writer: &mut dyn io::Write, _value: u64) -> Result<()> {
            Err(Error)
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = DummyWriter;
    let mut formatter = DummyFormatter;
    let serializer = Serializer {
        writer: &mut writer,
        formatter: &mut formatter,
    };

    let map_key_serializer = MapKeySerializer { ser: &mut serializer };

    let _ = map_key_serializer.serialize_u64(u64::MAX);
}

