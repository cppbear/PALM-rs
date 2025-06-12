// Answer 0

#[derive(Default)]
struct MockFormatter {
    begin_called: bool,
    end_called: bool,
}

impl MockFormatter {
    fn begin_string(&mut self, _writer: &mut Vec<u8>) -> Result<(), std::io::Error> {
        self.begin_called = true;
        Ok(())
    }

    fn write_u32(&mut self, _writer: &mut Vec<u8>, _value: u32) -> Result<(), std::io::Error> {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to write"))
    }

    fn end_string(&mut self, _writer: &mut Vec<u8>) -> Result<(), std::io::Error> {
        self.end_called = true;
        Ok(())
    }
}

struct MockSerializer {
    writer: Vec<u8>,
    formatter: MockFormatter,
}

impl Default for MockSerializer {
    fn default() -> Self {
        Self {
            writer: Vec::new(),
            formatter: MockFormatter::default(),
        }
    }
}

impl MockSerializer {
    fn serialize_u32(&mut self, value: u32) -> Result<(), std::io::Error> {
        self.formatter
            .begin_string(&mut self.writer)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "begin_string error"))?;
        self.formatter
            .write_u32(&mut self.writer, value)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "write_u32 error"))?;
        self.formatter
            .end_string(&mut self.writer)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "end_string error"))?;
        Ok(())
    }
}

#[test]
fn test_serialize_u32_should_return_err_on_write_failure() {
    let mut serializer = MockSerializer::default();
    let result = serializer.serialize_u32(42);
    assert!(result.is_err());
}

#[test]
fn test_serialize_u32_should_call_begin_and_end() {
    let mut serializer = MockSerializer::default();
    let _ = serializer.serialize_u32(42);
    assert!(serializer.formatter.begin_called);
    assert!(serializer.formatter.end_called);
}

