// Answer 0

#[derive(Debug)]
struct Content {
    kind: String,
}

impl Content {
    const Unit: Self = Content { kind: "Unit".to_string() };
}

struct TestSerializer;

impl TestSerializer {
    fn serialize_unit(self) -> Result<Content, &'static str> {
        Ok(Content::Unit)
    }
}

#[test]
fn test_serialize_unit() {
    let serializer = TestSerializer;
    let result = serializer.serialize_unit();
    assert_eq!(result, Ok(Content::Unit));
}

