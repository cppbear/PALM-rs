// Answer 0

#[derive(Debug)]
struct MockWriter {
    data: Vec<u8>,
}

impl MockWriter {
    fn new() -> Self {
        MockWriter { data: Vec::new() }
    }

    fn write(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }
}

struct Serializer<W> {
    writer: W,
}

impl<W> Serializer<W> {
    fn new(writer: W) -> Self {
        Serializer { writer }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }
}

#[test]
fn test_into_inner_with_mock_writer() {
    let mock_writer = MockWriter::new();
    let serializer = Serializer::new(mock_writer);
    let inner = serializer.into_inner();
    assert_eq!(format!("{:?}", inner), "MockWriter { data: [] }");
}

#[test]
fn test_into_inner_with_written_data() {
    let mut mock_writer = MockWriter::new();
    mock_writer.write(b"Hello, World!");
    let serializer = Serializer::new(mock_writer);
    let inner = serializer.into_inner();
    assert_eq!(format!("{:?}", inner), "MockWriter { data: [72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33] }");
}

