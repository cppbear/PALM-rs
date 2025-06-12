// Answer 0

#[test]
fn test_serialize_f64_infinite() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize, std::io::Error> {
            Ok(0)
        }

        fn flush(&mut self) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_f64(&mut self, _: &mut dyn io::Write, _: f64) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    let result = serializer.serialize_f64(f64::INFINITY);
    assert_eq!(result, Err(float_key_must_be_finite()));
}

#[test]
fn test_serialize_f64_nan() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize, std::io::Error> {
            Ok(0)
        }

        fn flush(&mut self) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_f64(&mut self, _: &mut dyn io::Write, _: f64) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    let result = serializer.serialize_f64(f64::NAN);
    assert_eq!(result, Err(float_key_must_be_finite()));
}

