// Answer 0

#[test]
fn test_serialize_i32_positive_value() {
    struct TestFormatter;

    impl TestFormatter {
        fn write_i32(&mut self, _: &mut dyn io::Write, _: i32) -> Result<()> {
            Ok(())
        }
    }

    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(1)
        }

        fn write_all(&mut self, _: &[u8]) -> Result<()> {
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter;
    let formatter = TestFormatter;

    let serializer = Serializer {
        writer,
        formatter,
    };

    let result = serializer.serialize_i32(42);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_i32_negative_value() {
    struct TestFormatter;

    impl TestFormatter {
        fn write_i32(&mut self, _: &mut dyn io::Write, _: i32) -> Result<()> {
            Ok(())
        }
    }

    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(1)
        }

        fn write_all(&mut self, _: &[u8]) -> Result<()> {
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter;
    let formatter = TestFormatter;

    let serializer = Serializer {
        writer,
        formatter,
    };

    let result = serializer.serialize_i32(-42);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_i32_zero_value() {
    struct TestFormatter;

    impl TestFormatter {
        fn write_i32(&mut self, _: &mut dyn io::Write, _: i32) -> Result<()> {
            Ok(())
        }
    }

    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(1)
        }

        fn write_all(&mut self, _: &[u8]) -> Result<()> {
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter;
    let formatter = TestFormatter;

    let serializer = Serializer {
        writer,
        formatter,
    };

    let result = serializer.serialize_i32(0);
    assert!(result.is_ok());
}

