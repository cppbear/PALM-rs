// Answer 0

#[derive(Debug)]
struct MySerializer;

impl MySerializer {
    fn bad_type(&self, _unsupported: Unsupported) -> Result<(), String> {
        Err("Unsupported type".to_string())
    }
}

trait Serialize {}

#[derive(Debug)]
struct MyType;

impl Serialize for MyType {}

#[test]
fn test_serialize_some() {
    let serializer = MySerializer;
    let result: Result<(), String> = serializer.serialize_some(&MyType);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Unsupported type");
}

#[test]
fn test_serialize_some_with_nil() {
    let serializer = MySerializer;
    let result: Result<(), String> = serializer.serialize_some(&());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Unsupported type");
}

