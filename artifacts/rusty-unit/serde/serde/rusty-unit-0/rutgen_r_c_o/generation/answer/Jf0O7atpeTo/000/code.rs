// Answer 0

#[derive(Debug)]
struct TestSerializer;

impl TestSerializer {
    fn bad_type(&self, _: Unsupported) -> ! {
        panic!("Unsupported type")
    }
}

enum Unsupported {
    TupleStruct,
}

impl Serialize for TestSerializer {
    type SerializeTupleStruct = ();
    type Error = ();

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(self.bad_type(Unsupported::TupleStruct))
    }
}

#[test]
#[should_panic(expected = "Unsupported type")]
fn test_serialize_tuple_struct() {
    let serializer = TestSerializer;
    serializer.serialize_tuple_struct("Test", 1).unwrap();
}

