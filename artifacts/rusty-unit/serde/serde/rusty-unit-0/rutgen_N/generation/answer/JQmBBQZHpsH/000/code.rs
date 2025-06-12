// Answer 0

#[derive(Debug)]
struct MySerializer;

impl MySerializer {
    fn bad_type(_: Unsupported) -> String {
        "Bad type".to_string()
    }
}

trait Serializer {
    type Ok;
    type Error;
    
    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error>;
}

impl Serializer for MySerializer {
    type Ok = ();
    type Error = String;

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Err(Self::bad_type(Unsupported::Integer))
    }
}

#[derive(Debug)]
struct Unsupported;

impl Unsupported {
    const Integer: Unsupported = Unsupported;
}

#[test]
fn test_serialize_i32_error() {
    let serializer = MySerializer;
    let result = serializer.serialize_i32(42);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "Bad type");
}

