// Answer 0

#[derive(Debug)]
struct Content {
    value: bool,
}

impl Content {
    fn Bool(v: bool) -> Self {
        Content { value: v }
    }
}

#[derive(Debug)]
struct Serializer;

impl Serializer {
    fn serialize_bool(self, v: bool) -> Result<Content, &'static str> {
        Ok(Content::Bool(v))
    }
}

#[test]
fn test_serialize_bool_true() {
    let serializer = Serializer;
    let result = serializer.serialize_bool(true).unwrap();
    assert_eq!(result.value, true);
}

#[test]
fn test_serialize_bool_false() {
    let serializer = Serializer;
    let result = serializer.serialize_bool(false).unwrap();
    assert_eq!(result.value, false);
}

#[test]
#[should_panic(expected = "Not meant to panic")]
fn test_serialize_bool_invalid() {
    // This test case is intentionally left to represent a method that doesn't panic.
    // If your implementation had a way to fail, it would go here.
    let serializer = Serializer;
    let _ = serializer.serialize_bool(true); // No panic expected
}

