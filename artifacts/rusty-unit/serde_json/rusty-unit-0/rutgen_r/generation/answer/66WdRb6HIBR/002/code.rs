// Answer 0

fn test_serialize_bool_ok() {
    struct Formatter;
    struct Writer;

    struct Serializer {
        writer: Writer,
        formatter: Formatter,
    }

    impl Formatter {
        fn begin_string(&self, _writer: &mut Writer) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn write_bool(&self, _writer: &mut Writer, _value: bool) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_string(&self, _writer: &mut Writer) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    impl Serializer {
        fn serialize_bool(&mut self, value: bool) -> Result<(), std::io::Error> {
            self.formatter.begin_string(&mut self.writer)?;
            self.formatter.write_bool(&mut self.writer, value)?;
            self.formatter.end_string(&mut self.writer)
        }
    }

    let mut serializer = Serializer {
        writer: Writer,
        formatter: Formatter,
    };

    let result = serializer.serialize_bool(true);
    assert!(result.is_ok());
}

fn test_serialize_bool_write_bool_err() {
    struct Formatter;
    struct Writer;

    struct Serializer {
        writer: Writer,
        formatter: Formatter,
    }

    impl Formatter {
        fn begin_string(&self, _writer: &mut Writer) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn write_bool(&self, _writer: &mut Writer, _value: bool) -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "write error"))
        }

        fn end_string(&self, _writer: &mut Writer) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    impl Serializer {
        fn serialize_bool(&mut self, value: bool) -> Result<(), std::io::Error> {
            self.formatter.begin_string(&mut self.writer)?;
            self.formatter.write_bool(&mut self.writer, value)?;
            self.formatter.end_string(&mut self.writer)
        }
    }

    let mut serializer = Serializer {
        writer: Writer,
        formatter: Formatter,
    };

    let result = serializer.serialize_bool(true);
    assert!(result.is_err());
}

fn test_serialize_bool_begin_string_err() {
    struct Formatter;
    struct Writer;

    struct Serializer {
        writer: Writer,
        formatter: Formatter,
    }

    impl Formatter {
        fn begin_string(&self, _writer: &mut Writer) -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "begin string error"))
        }

        fn write_bool(&self, _writer: &mut Writer, _value: bool) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_string(&self, _writer: &mut Writer) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    impl Serializer {
        fn serialize_bool(&mut self, value: bool) -> Result<(), std::io::Error> {
            self.formatter.begin_string(&mut self.writer)?;
            self.formatter.write_bool(&mut self.writer, value)?;
            self.formatter.end_string(&mut self.writer)
        }
    }

    let mut serializer = Serializer {
        writer: Writer,
        formatter: Formatter,
    };

    let result = serializer.serialize_bool(true);
    assert!(result.is_err());
}

fn test_serialize_bool_end_string_err() {
    struct Formatter;
    struct Writer;

    struct Serializer {
        writer: Writer,
        formatter: Formatter,
    }

    impl Formatter {
        fn begin_string(&self, _writer: &mut Writer) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn write_bool(&self, _writer: &mut Writer, _value: bool) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_string(&self, _writer: &mut Writer) -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "end string error"))
        }
    }

    impl Serializer {
        fn serialize_bool(&mut self, value: bool) -> Result<(), std::io::Error> {
            self.formatter.begin_string(&mut self.writer)?;
            self.formatter.write_bool(&mut self.writer, value)?;
            self.formatter.end_string(&mut self.writer)
        }
    }

    let mut serializer = Serializer {
        writer: Writer,
        formatter: Formatter,
    };

    let result = serializer.serialize_bool(true);
    assert!(result.is_err());
}

