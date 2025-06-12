// Answer 0

#[test]
fn test_serialize_i64_success() {
    struct Formatter;
    struct Writer;
    struct Ser {
        formatter: Formatter,
        writer: Writer,
    }

    struct TestSerializer {
        ser: Ser,
    }

    impl Formatter {
        fn begin_string(&self, writer: &mut Writer) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn write_i64(&self, writer: &mut Writer, value: i64) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_string(&self, writer: &mut Writer) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    impl TestSerializer {
        fn serialize_i64(self, value: i64) -> Result<(), std::io::Error> {
            self.ser.formatter.begin_string(&mut self.ser.writer)?;
            self.ser.formatter.write_i64(&mut self.ser.writer, value)?;
            self.ser.formatter.end_string(&mut self.ser.writer)?;
            Ok(())
        }
    }

    let formatter = Formatter;
    let writer = Writer;
    let ser = Ser { formatter, writer };
    let serializer = TestSerializer { ser };

    let result = serializer.serialize_i64(42);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_i64_begin_string_fail() {
    struct Formatter;
    struct Writer;
    struct Ser {
        formatter: Formatter,
        writer: Writer,
    }

    struct TestSerializer {
        ser: Ser,
    }

    impl Formatter {
        fn begin_string(&self, _writer: &mut Writer) -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Begin string error"))
        }

        fn write_i64(&self, _writer: &mut Writer, _value: i64) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_string(&self, _writer: &mut Writer) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    impl TestSerializer {
        fn serialize_i64(self, value: i64) -> Result<(), std::io::Error> {
            self.ser.formatter.begin_string(&mut self.ser.writer)?;
            self.ser.formatter.write_i64(&mut self.ser.writer, value)?;
            self.ser.formatter.end_string(&mut self.ser.writer)?;
            Ok(())
        }
    }

    let formatter = Formatter;
    let writer = Writer;
    let ser = Ser { formatter, writer };
    let serializer = TestSerializer { ser };

    let _ = serializer.serialize_i64(42);
}

#[test]
#[should_panic]
fn test_serialize_i64_write_i64_fail() {
    struct Formatter;
    struct Writer;
    struct Ser {
        formatter: Formatter,
        writer: Writer,
    }

    struct TestSerializer {
        ser: Ser,
    }

    impl Formatter {
        fn begin_string(&self, _writer: &mut Writer) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn write_i64(&self, _writer: &mut Writer, _value: i64) -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Write i64 error"))
        }

        fn end_string(&self, _writer: &mut Writer) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    impl TestSerializer {
        fn serialize_i64(self, value: i64) -> Result<(), std::io::Error> {
            self.ser.formatter.begin_string(&mut self.ser.writer)?;
            self.ser.formatter.write_i64(&mut self.ser.writer, value)?;
            self.ser.formatter.end_string(&mut self.ser.writer)?;
            Ok(())
        }
    }

    let formatter = Formatter;
    let writer = Writer;
    let ser = Ser { formatter, writer };
    let serializer = TestSerializer { ser };

    let _ = serializer.serialize_i64(42);
}

#[test]
#[should_panic]
fn test_serialize_i64_end_string_fail() {
    struct Formatter;
    struct Writer;
    struct Ser {
        formatter: Formatter,
        writer: Writer,
    }

    struct TestSerializer {
        ser: Ser,
    }

    impl Formatter {
        fn begin_string(&self, _writer: &mut Writer) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn write_i64(&self, _writer: &mut Writer, _value: i64) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_string(&self, _writer: &mut Writer) -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "End string error"))
        }
    }

    impl TestSerializer {
        fn serialize_i64(self, value: i64) -> Result<(), std::io::Error> {
            self.ser.formatter.begin_string(&mut self.ser.writer)?;
            self.ser.formatter.write_i64(&mut self.ser.writer, value)?;
            self.ser.formatter.end_string(&mut self.ser.writer)?;
            Ok(())
        }
    }

    let formatter = Formatter;
    let writer = Writer;
    let ser = Ser { formatter, writer };
    let serializer = TestSerializer { ser };

    let _ = serializer.serialize_i64(42);
}

