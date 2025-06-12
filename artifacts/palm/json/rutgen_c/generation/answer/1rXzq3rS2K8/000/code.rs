// Answer 0

#[test]
fn test_serialize_f32_valid() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn begin_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_f32(&mut self, _: &mut dyn io::Write, _: f32) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { output: Vec::new() };
    let formatter = TestFormatter;
    let mut serializer = MapKeySerializer { ser: &mut Serializer { writer, formatter } };

    let result = serializer.serialize_f32(1.0);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_f32_non_finite() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn begin_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_f32(&mut self, _: &mut dyn io::Write, _: f32) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { output: Vec::new() };
    let formatter = TestFormatter;
    let mut serializer = MapKeySerializer { ser: &mut Serializer { writer, formatter } };

    // Here we pass a non-finite value (NaN)
    let _ = serializer.serialize_f32(f32::NAN);
}

