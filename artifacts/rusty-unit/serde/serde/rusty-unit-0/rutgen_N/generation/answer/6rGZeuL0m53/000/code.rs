// Answer 0

#[derive(Debug)]
struct Serializer;

impl Serializer {
    fn bad_type(&self, _: Unsupported) -> ! {
        panic!("Unsupported type")
    }
    
    fn serialize_bool(self, _: bool) -> Result<(), ()> {
        Err(self.bad_type(Unsupported::Boolean))
    }
}

#[derive(Debug)]
struct Unsupported {
    kind: String,
}

impl Unsupported {
    fn boolean() -> Self {
        Unsupported {
            kind: String::from("Boolean"),
        }
    }
}

#[test]
#[should_panic(expected = "Unsupported type")]
fn test_serialize_bool() {
    let serializer = Serializer;
    let _result = serializer.serialize_bool(true);
}

