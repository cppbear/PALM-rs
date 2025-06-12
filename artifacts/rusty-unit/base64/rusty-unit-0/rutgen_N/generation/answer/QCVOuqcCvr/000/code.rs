// Answer 0

#[derive(Debug)]
struct DummyReader {
    data: Vec<u8>,
    // other fields as needed
}

impl DummyReader {
    fn new(data: Vec<u8>) -> Self {
        DummyReader { data }
    }
}

struct DecoderReader<R> {
    inner: R,
}

impl<R> DecoderReader<R> {
    fn new(inner: R) -> Self {
        DecoderReader { inner }
    }

    pub fn into_inner(self) -> R {
        self.inner
    }
}

#[test]
fn test_into_inner() {
    let bytes = vec![1, 2, 3, 4];
    let dummy_reader = DummyReader::new(bytes.clone());
    let decoder_reader = DecoderReader::new(dummy_reader);

    let inner = decoder_reader.into_inner();
    assert_eq!(inner.data, bytes);
}

#[test]
fn test_into_inner_empty() {
    let bytes = Vec::new();
    let dummy_reader = DummyReader::new(bytes.clone());
    let decoder_reader = DecoderReader::new(dummy_reader);

    let inner = decoder_reader.into_inner();
    assert_eq!(inner.data, bytes);
}

