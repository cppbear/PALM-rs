// Answer 0

#[test]
fn test_end_serializes_tuple_struct() {
    struct DummyMap {
        value: Option<Content>,
    }

    impl ser::SerializeMap for DummyMap {
        type Ok = ();
        type Error = ();

        fn serialize_value(&mut self, value: &Content) -> Result<(), Self::Error> {
            self.value = Some(value.clone());
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = DummyMap { value: None };
    let name = "TestTupleStruct";
    let fields = vec![Content::U32(42), Content::String("Hello".to_string())];
    
    let mut serializer = SerializeTupleVariantAsMapValue { map, name, fields };

    let result = serializer.end();
    
    assert!(result.is_ok());
    assert_eq!(serializer.map.value, Some(Content::TupleStruct(name, fields)));
}

#[test]
#[should_panic]
fn test_end_serializes_tuple_struct_panics_on_error() {
    struct ErrorMap;

    impl ser::SerializeMap for ErrorMap {
        type Ok = ();
        type Error = ();

        fn serialize_value(&mut self, _: &Content) -> Result<(), Self::Error> {
            panic!("Forced panic");
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let map = ErrorMap;
    let name = "TestTupleStruct";
    let fields = vec![Content::U32(42)];
    
    let mut serializer = SerializeTupleVariantAsMapValue { map, name, fields };

    let _ = serializer.end();
}

