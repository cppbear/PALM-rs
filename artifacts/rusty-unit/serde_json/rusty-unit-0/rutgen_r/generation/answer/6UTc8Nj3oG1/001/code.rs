// Answer 0

#[test]
fn test_serialize_u16_success() {
    struct TestFormatter;

    impl TestFormatter {
        fn write_u16<W>(&self, writer: &mut W, value: u16) -> Result<()>
        where
            W: std::io::Write,
        {
            writer.write_all(&value.to_le_bytes())?;
            Ok(())
        }
    }

    struct TestSerializer {
        formatter: TestFormatter,
        writer: Vec<u8>,
    }

    impl TestSerializer {
        fn new() -> Self {
            Self {
                formatter: TestFormatter,
                writer: Vec::new(),
            }
        }

        fn serialize_u16(self, value: u16) -> Result<()> {
            self.formatter
                .write_u16(&mut self.writer, value)
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "write error"))
        }
    }

    let serializer = TestSerializer::new();
    let result = serializer.serialize_u16(65535);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_u16_zero() {
    struct TestFormatter;

    impl TestFormatter {
        fn write_u16<W>(&self, writer: &mut W, value: u16) -> Result<()>
        where
            W: std::io::Write,
        {
            writer.write_all(&value.to_le_bytes())?;
            Ok(())
        }
    }

    struct TestSerializer {
        formatter: TestFormatter,
        writer: Vec<u8>,
    }

    impl TestSerializer {
        fn new() -> Self {
            Self {
                formatter: TestFormatter,
                writer: Vec::new(),
            }
        }

        fn serialize_u16(self, value: u16) -> Result<()> {
            self.formatter
                .write_u16(&mut self.writer, value)
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "write error"))
        }
    }

    let serializer = TestSerializer::new();
    let result = serializer.serialize_u16(0);
    assert!(result.is_ok());
}

#[should_panic]
#[test]
fn test_serialize_u16_write_error() {
    struct TestFormatter;

    impl TestFormatter {
        fn write_u16<W>(&self, writer: &mut W, value: u16) -> Result<()>
        where
            W: std::io::Write,
        {
            // Simulate an error by writing to an unreachable sink
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "simulated write error").into());
        }
    }

    struct TestSerializer {
        formatter: TestFormatter,
        writer: Vec<u8>,
    }

    impl TestSerializer {
        fn new() -> Self {
            Self {
                formatter: TestFormatter,
                writer: Vec::new(),
            }
        }

        fn serialize_u16(self, value: u16) -> Result<()> {
            self.formatter
                .write_u16(&mut self.writer, value)
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "write error"))?;
            Ok(())
        }
    }

    let serializer = TestSerializer::new();
    serializer.serialize_u16(1).unwrap(); // This should panic
}

