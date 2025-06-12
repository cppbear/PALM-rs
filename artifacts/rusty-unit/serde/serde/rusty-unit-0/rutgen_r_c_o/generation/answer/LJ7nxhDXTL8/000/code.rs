// Answer 0

#[derive(Debug)]
struct BadTypeError;

struct Serializer;

impl Serializer {
    fn bad_type(&self, _: Unsupported) -> Result<(), BadTypeError> {
        Err(BadTypeError)
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<(), BadTypeError> {
        self.bad_type(Unsupported::Sequence)
    }
}

#[derive(Debug)]
enum Unsupported {
    Sequence,
}

#[test]
fn test_serialize_seq_returns_error() {
    let serializer = Serializer;
    let result = serializer.serialize_seq(None);
    assert!(result.is_err());
}

