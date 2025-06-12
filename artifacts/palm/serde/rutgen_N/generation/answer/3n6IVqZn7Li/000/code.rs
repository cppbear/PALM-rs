// Answer 0

#[derive(Debug)]
struct Serializer;

impl Serializer {
    fn bad_type(&self, _: Unsupported) -> String {
        "Bad type error".to_string()
    }
}

#[derive(Debug)]
struct Unsupported {
    kind: &'static str,
}

impl Unsupported {
    const Integer: &'static str = "Integer";
}

impl Serializer {
    type Ok = ();
    type Error = String;

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }
}

#[test]
fn test_serialize_i32() {
    let serializer = Serializer;
    let result = serializer.serialize_i32(42);
    assert_eq!(result, Err("Bad type error".to_string()));
}

