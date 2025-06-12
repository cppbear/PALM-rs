// Answer 0

#[test]
fn test_serialize_i8_positive() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
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

    impl Formatter for MockFormatter {
        fn write_i8(&mut self, writer: &mut dyn io::Write, value: i8) -> Result<()> {
            writer.write_all(&[value as u8])
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    serializer.serialize_i8(42).unwrap();

    assert_eq!(serializer.writer.output, vec![42]);
}

#[test]
fn test_serialize_i8_negative() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
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

    impl Formatter for MockFormatter {
        fn write_i8(&mut self, writer: &mut dyn io::Write, value: i8) -> Result<()> {
            writer.write_all(&[value as u8])
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    serializer.serialize_i8(-42).unwrap();

    assert_eq!(serializer.writer.output, vec![214]); // -42 as u8 is 214
}

#[test]
fn test_serialize_i8_zero() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
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

    impl Formatter for MockFormatter {
        fn write_i8(&mut self, writer: &mut dyn io::Write, value: i8) -> Result<()> {
            writer.write_all(&[value as u8])
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    serializer.serialize_i8(0).unwrap();

    assert_eq!(serializer.writer.output, vec![0]);
}

