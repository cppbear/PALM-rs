// Answer 0

#[derive(Debug)]
struct MyStruct;

impl MyStruct {
    fn serialize_none(self) -> Result<(), String> {
        Err("key must be a string".into())
    }
}

#[test]
fn test_serialize_none() {
    let my_struct = MyStruct;
    let result = my_struct.serialize_none();
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "key must be a string");
}

