// Answer 0

#[derive(Default)]
struct TestFormatter {
    should_panic: bool,
}

impl TestFormatter {
    fn write_null(&self, _writer: &mut Vec<u8>) -> Result<(), std::io::Error> {
        if self.should_panic {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "panic"))
        } else {
            Ok(())
        }
    }
}

struct Serializer {
    formatter: TestFormatter,
    writer: Vec<u8>,
}

impl Serializer {
    fn new(formatter: TestFormatter) -> Self {
        Serializer {
            formatter,
            writer: Vec::new(),
        }
    }

    fn serialize_unit(self) -> Result<(), std::io::Error> {
        self.formatter
            .write_null(&mut self.writer)
            .map_err(|e| e)
    }
}

#[test]
fn test_serialize_unit_success() {
    let formatter = TestFormatter { should_panic: false };
    let serializer = Serializer::new(formatter);
    let result = serializer.serialize_unit();
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "panic")]
fn test_serialize_unit_panic() {
    let formatter = TestFormatter { should_panic: true };
    let serializer = Serializer::new(formatter);
    let result = serializer.serialize_unit().unwrap(); // This will panic
}

