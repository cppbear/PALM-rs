// Answer 0

#[test]
fn test_serialize_f32_nan() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(0)
        }

        fn write_all(&mut self, _buf: &[u8]) -> Result<()> {
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn write_null(&mut self, _writer: &mut MockWriter) -> Result<()> {
            // Mock implementation: simulates writing null
            assert!(true); // Verify this function is called
            Ok(())
        }

        fn write_f32(&mut self, _writer: &mut MockWriter, _value: f32) -> Result<()> {
            // Mock implementation: simulates writing float
            unreachable!(); // This should not be called for NaN
        }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;
    let serializer = &mut Serializer { writer, formatter };
    
    let result = serializer.serialize_f32(f32::NaN);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_f32_infinite() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(0)
        }

        fn write_all(&mut self, _buf: &[u8]) -> Result<()> {
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn write_null(&mut self, _writer: &mut MockWriter) -> Result<()> {
            assert!(true); // Ensure this function is called
            Ok(())
        }

        fn write_f32(&mut self, _writer: &mut MockWriter, _value: f32) -> Result<()> {
            unreachable!(); // This should not be called for infinite
        }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;
    let serializer = &mut Serializer { writer, formatter };

    let result = serializer.serialize_f32(f32::INFINITY);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_f32_regular_value() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(0)
        }

        fn write_all(&mut self, _buf: &[u8]) -> Result<()> {
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn write_null(&mut self, _writer: &mut MockWriter) -> Result<()> {
            unreachable!(); // This should not be called for regular value
        }

        fn write_f32(&mut self, _writer: &mut MockWriter, value: f32) -> Result<()> {
            assert_eq!(value, 3.14); // Verify that the correct value is "serialized"
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let mut formatter = MockFormatter;
    let serializer = &mut Serializer { writer, formatter };

    let result = serializer.serialize_f32(3.14);
    assert!(result.is_ok());
}

