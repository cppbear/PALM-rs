// Answer 0

#[derive(Default)]
struct DummyFormatter;

impl DummyFormatter {
    fn write_byte_array(&self, writer: &mut Vec<u8>, value: &[u8]) -> Result<(), std::io::Error> {
        writer.extend_from_slice(value);
        Ok(())
    }
}

struct Serializer<W> {
    formatter: DummyFormatter,
    writer: W,
}

impl<W> Serializer<W> {
    fn new(writer: W) -> Self {
        Serializer {
            formatter: DummyFormatter::default(),
            writer,
        }
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<(), std::io::Error> {
        self.formatter
            .write_byte_array(&mut self.writer, value)
            .map_err(|e| e)
    }
}

#[test]
fn test_serialize_bytes() {
    let mut output = Vec::new();
    let serializer = Serializer::new(&mut output);
    let result = serializer.serialize_bytes(&[1, 2, 3]);

    assert!(result.is_ok());
    assert_eq!(output, &[1, 2, 3]);
}

#[test]
fn test_serialize_empty_bytes() {
    let mut output = Vec::new();
    let serializer = Serializer::new(&mut output);
    let result = serializer.serialize_bytes(&[]);

    assert!(result.is_ok());
    assert!(output.is_empty());
}

