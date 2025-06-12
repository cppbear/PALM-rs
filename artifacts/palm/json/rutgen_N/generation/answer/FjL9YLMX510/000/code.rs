// Answer 0

#[derive(Debug)]
struct SerializeSeq;

impl SerializeSeq {
    fn new() -> Self {
        SerializeSeq
    }
}

struct Serializer;

impl Serializer {
    fn serialize_seq(self, _len: Option<usize>) -> Result<SerializeSeq, String> {
        Err("key must be a string".to_string())
    }
}

#[test]
fn test_serialize_seq_err() {
    let serializer = Serializer;
    let result = serializer.serialize_seq(Some(3));
    
    assert_eq!(result, Err("key must be a string".to_string()));
}

