// Answer 0

#[test]
fn test_serialize_finite_f64() {
    struct TestFormatter {
        called_begin: bool,
        called_write: bool,
        called_end: bool,
    }
    
    impl TestFormatter {
        fn new() -> Self {
            TestFormatter {
                called_begin: false,
                called_write: false,
                called_end: false,
            }
        }

        fn begin_string(&mut self, _: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            self.called_begin = true;
            Ok(())
        }

        fn write_f64(&mut self, _: &mut dyn std::io::Write, _: f64) -> Result<(), std::io::Error> {
            self.called_write = true;
            Ok(())
        }

        fn end_string(&mut self, _: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            self.called_end = true;
            Ok(())
        }
    }

    struct TestSerializer {
        writer: Vec<u8>,
        formatter: TestFormatter,
    }

    impl TestSerializer {
        fn new() -> Self {
            TestSerializer {
                writer: vec![],
                formatter: TestFormatter::new(),
            }
        }

        fn serialize_f64(&mut self, value: f64) -> Result<()> {
            if !value.is_finite() {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "value must be finite"));
            }

            self.formatter.begin_string(&mut self.writer)?;
            self.formatter.write_f64(&mut self.writer, value)?;
            self.formatter.end_string(&mut self.writer)?;
            Ok(())
        }
    }

    let mut serializer = TestSerializer::new();
    let result = serializer.serialize_f64(3.14);
    assert!(result.is_ok());
    assert!(serializer.formatter.called_begin);
    assert!(serializer.formatter.called_write);
    assert!(serializer.formatter.called_end);
}

#[test]
#[should_panic]
fn test_serialize_infinite_f64() {
    struct TestFormatter {
        called_begin: bool,
    }
    
    impl TestFormatter {
        fn new() -> Self {
            TestFormatter {
                called_begin: false,
            }
        }

        fn begin_string(&mut self, _: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            self.called_begin = true;
            Ok(())
        }
    }

    struct TestSerializer {
        writer: Vec<u8>,
        formatter: TestFormatter,
    }

    impl TestSerializer {
        fn new() -> Self {
            TestSerializer {
                writer: vec![],
                formatter: TestFormatter::new(),
            }
        }

        fn serialize_f64(&mut self, value: f64) -> Result<()> {
            if !value.is_finite() {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "value must be finite"));
            }

            self.formatter.begin_string(&mut self.writer)?;
            Ok(())
        }
    }

    let mut serializer = TestSerializer::new();
    let _ = serializer.serialize_f64(f64::INFINITY);
}

