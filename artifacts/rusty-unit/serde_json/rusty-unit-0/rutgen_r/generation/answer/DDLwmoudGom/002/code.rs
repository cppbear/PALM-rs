// Answer 0

#[derive(Default)]
struct MockFormatter {
    should_fail_begin: bool,
    should_fail_write: bool,
}

impl MockFormatter {
    fn begin_string(&mut self, _writer: &mut Vec<u8>) -> Result<(), std::io::Error> {
        if self.should_fail_begin {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "begin_string error"))
        } else {
            Ok(())
        }
    }

    fn write_u16(&mut self, _writer: &mut Vec<u8>, _value: u16) -> Result<(), std::io::Error> {
        if self.should_fail_write {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "write_u16 error"))
        } else {
            Ok(())
        }
    }

    fn end_string(&mut self, _writer: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
}

struct Serializer {
    writer: Vec<u8>,
    formatter: MockFormatter,
}

impl Serializer {
    fn new() -> Self {
        Serializer {
            writer: Vec::new(),
            formatter: MockFormatter::default(),
        }
    }

    fn serialize_u16(&mut self, value: u16) -> Result<(), std::io::Error> {
        self.formatter.begin_string(&mut self.writer)?;
        self.formatter.write_u16(&mut self.writer, value)?;
        self.formatter.end_string(&mut self.writer)
    }
}

#[test]
fn test_serialize_u16_success() {
    let mut serializer = Serializer::new();
    assert!(serializer.serialize_u16(42).is_ok());
}

#[test]
fn test_serialize_u16_fail_begin() {
    let mut serializer = Serializer::new();
    serializer.formatter.should_fail_begin = true;
    assert!(serializer.serialize_u16(42).is_err());
}

#[test]
fn test_serialize_u16_fail_write() {
    let mut serializer = Serializer::new();
    serializer.formatter.should_fail_write = true;
    assert!(serializer.serialize_u16(42).is_err());
}

