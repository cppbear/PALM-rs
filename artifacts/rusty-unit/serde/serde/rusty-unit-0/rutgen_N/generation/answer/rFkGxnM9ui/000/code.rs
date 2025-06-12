// Answer 0

#[derive(Debug)]
struct TestSerializer;

impl TestSerializer {
    fn bad_type(&self, _: Unsupported) -> Result<(), String> {
        Err("Unsupported type".to_string())
    }
}

impl serde::ser::Serializer for TestSerializer {
    type Ok = ();
    type Error = String;
    type SerializeSeq = ();
    type SerializeTuple = ();
    type SerializeTupleStruct = ();
    type SerializeTupleVariant = ();
    type SerializeMap = ();
    type SerializeStruct = ();
    type SerializeStructVariant = ();
    
    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Integer))
    }
    
    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    // add other required methods with unimplemented!() here
}

#[test]
fn test_serialize_u8() {
    let serializer = TestSerializer;
    let result = serializer.serialize_u8(42);
    assert_eq!(result.err(), Some("Unsupported type".to_string()));
}

