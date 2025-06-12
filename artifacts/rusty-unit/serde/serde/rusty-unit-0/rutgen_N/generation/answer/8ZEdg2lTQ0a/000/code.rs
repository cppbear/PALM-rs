// Answer 0

#[test]
fn test_serialization_of_tuple_struct() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(_: Unsupported) -> ! {
            panic!("bad type")
        }
    }

    impl serde::Serializer for TestSerializer {
        type SerializeTupleStruct = ();
        type Error = ();

        fn serialize_tuple_struct(
            self,
            _: &'static str,
            _: usize,
        ) -> Result<Self::SerializeTupleStruct, Self::Error> {
            Err(Self::bad_type(Unsupported::TupleStruct))
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_tuple_struct("Test", 1);
    
    assert!(result.is_err());
}

#[derive(Debug)]
enum Unsupported {
    TupleStruct,
}

