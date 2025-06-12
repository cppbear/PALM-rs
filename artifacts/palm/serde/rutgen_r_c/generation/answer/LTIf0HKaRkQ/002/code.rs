// Answer 0

#[test]
fn test_end_serializing_success() {
    struct MockMap {
        called: bool,
    }

    impl ser::SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

        fn serialize_value(&mut self, value: &Content) -> Result<(), Self::Error> {
            self.called = true;
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            assert!(self.called);
            Ok(())
        }
    }

    let mut map = MockMap { called: false };
    let name = "test_variant";
    let fields = vec![Content::U32(42)];

    let mut serializer = SerializeTupleVariantAsMapValue {
        map,
        name,
        fields,
    };

    let result = serializer.end();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_end_serializing_failure() {
    struct MockMap {
        called: bool,
    }

    impl ser::SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

        fn serialize_value(&mut self, _value: &Content) -> Result<(), Self::Error> {
            Err(Error)
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockMap { called: false };
    let name = "test_variant";
    let fields = vec![Content::U32(42)];

    let serializer = SerializeTupleVariantAsMapValue {
        map,
        name,
        fields,
    };

    let _ = serializer.end(); // This should trigger a panic due to the error in serialize_value
}

