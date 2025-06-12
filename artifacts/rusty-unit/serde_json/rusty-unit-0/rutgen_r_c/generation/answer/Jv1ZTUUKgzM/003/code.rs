// Answer 0

#[test]
fn test_serialize_f32_nan() {
    struct MockWriter;
    impl crate::io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            assert_eq!(buf, b"null");
            Ok(buf.len())
        }
        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            self.write(buf).map(|_| ())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;
    impl MockFormatter {
        fn write_null(&self, writer: &mut MockWriter) -> Result<()> {
            writer.write(b"null")?;
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;

    // Testing with NaN
    let result = (&mut writer, &formatter).serialize_f32(f32::NAN);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_f32_infinite() {
    struct MockWriter;
    impl crate::io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            assert_eq!(buf, b"null");
            Ok(buf.len())
        }
        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            self.write(buf).map(|_| ())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;
    impl MockFormatter {
        fn write_null(&self, writer: &mut MockWriter) -> Result<()> {
            writer.write(b"null")?;
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;

    // Testing with positive infinity
    let result = (&mut writer, &formatter).serialize_f32(f32::INFINITY);
    assert!(result.is_ok());

    // Testing with negative infinity
    let result = (&mut writer, &formatter).serialize_f32(f32::NEG_INFINITY);
    assert!(result.is_ok());
}

