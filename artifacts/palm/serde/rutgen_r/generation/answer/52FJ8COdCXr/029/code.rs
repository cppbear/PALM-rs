// Answer 0

#[derive(Debug)]
enum Content {
    UnitStruct(&'static str),
}

trait Serializer {
    type Ok;
    type Error;
    
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error>;
}

struct TestSerializer {
    result: String,
}

impl Serializer for TestSerializer {
    type Ok = ();
    type Error = ();

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.result.push_str(&format!("Serialized UnitStruct: {}", name));
        Ok(())
    }
}

#[test]
fn test_serialize_unit_struct() {
    let content = Content::UnitStruct("MyUnitStruct");
    let serializer = TestSerializer { result: String::new() };

    let _ = match content {
        Content::UnitStruct(name) => serializer.serialize_unit_struct(name),
        _ => unreachable!(),
    };

    assert_eq!(serializer.result, "Serialized UnitStruct: MyUnitStruct");
}

#[test]
#[should_panic]
fn test_serialize_unit_struct_panic() {
    let content = Content::UnitStruct("PanicStruct");
    let serializer = TestSerializer { result: String::new() };

    let _ = match content {
        Content::UnitStruct(_) => serializer.serialize_unit_struct("ShouldNotPanic"),
        _ => panic!("Expected UnitStruct but found something else."),
    };
}

