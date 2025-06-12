// Answer 0

#[derive(Debug)]
struct MockSerializer;

impl MockSerializer {
    fn bad_type(&self, _: Unsupported) -> Result<(), String> {
        Err("Bad type error".to_string())
    }
}

struct Unsupported;

impl MockSerializer {
    fn serialize_seq(self, _: Option<usize>) -> Result<(), String> {
        Err(self.bad_type(Unsupported))
    }
}

#[test]
fn test_serialize_seq_should_return_error() {
    let serializer = MockSerializer;
    let result = serializer.serialize_seq(None);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "Bad type error");
}

