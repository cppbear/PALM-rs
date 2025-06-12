// Answer 0

#[derive(Debug)]
struct Unsupported;

impl Unsupported {
    fn Sequence() -> Self {
        Unsupported
    }
}

#[derive(Debug)]
struct SerializeSeq;

struct Serde;

impl Serde {
    fn bad_type(_: Unsupported) -> Result<SerializeSeq, String> {
        Err(String::from("bad type"))
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<SerializeSeq, String> {
        Err(Self::bad_type(Unsupported::Sequence()))
    }
}

#[test]
fn test_serialize_seq_err() {
    let serde_instance = Serde;
    let result = serde_instance.serialize_seq(None);
    assert_eq!(result, Err(String::from("bad type")));
}

#[test]
fn test_serialize_seq_err_with_some() {
    let serde_instance = Serde;
    let result = serde_instance.serialize_seq(Some(1));
    assert_eq!(result, Err(String::from("bad type")));
}

