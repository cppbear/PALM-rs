// Answer 0

#[derive(Default)]
struct Formatter {
    start_called: bool,
    end_called: bool,
}

impl Formatter {
    fn begin_string(&mut self, writer: &mut Vec<u8>) -> Result<(), std::io::Error> {
        self.start_called = true;
        writer.push(b'"');
        Ok(())
    }

    fn write_i8(&mut self, writer: &mut Vec<u8>, value: i8) -> Result<(), std::io::Error> {
        if !self.start_called {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "start not called"));
        }
        writer.extend(value.to_string().bytes());
        Ok(())
    }

    fn end_string(&mut self, writer: &mut Vec<u8>) -> Result<(), std::io::Error> {
        if !self.start_called {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "start not called"));
        }
        writer.push(b'"');
        self.end_called = true;
        Ok(())
    }
}

struct Serializer {
    formatter: Formatter,
    writer: Vec<u8>,
}

impl Serializer {
    fn new() -> Self {
        Serializer {
            formatter: Formatter::default(),
            writer: Vec::new(),
        }
    }

    fn serialize_i8(self, value: i8) -> Result<(), std::io::Error> {
        self.formatter
            .begin_string(&mut self.writer)
            .map_err(|e| e)?;
        self.formatter
            .write_i8(&mut self.writer, value)
            .map_err(|e| e)?;
        self.formatter
            .end_string(&mut self.writer)
            .map_err(|e| e)?;
        Ok(())
    }
}

#[test]
fn test_serialize_i8_valid_value() {
    let serializer = Serializer::new();
    let result = serializer.serialize_i8(5);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_i8_negative_value() {
    let serializer = Serializer::new();
    let result = serializer.serialize_i8(-3);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_i8_boundary_value() {
    let serializer = Serializer::new();
    let result_zero = serializer.serialize_i8(0);
    let result_max = serializer.serialize_i8(i8::MAX);
    let result_min = serializer.serialize_i8(i8::MIN);
    
    assert!(result_zero.is_ok());
    assert!(result_max.is_ok());
    assert!(result_min.is_ok());
}

