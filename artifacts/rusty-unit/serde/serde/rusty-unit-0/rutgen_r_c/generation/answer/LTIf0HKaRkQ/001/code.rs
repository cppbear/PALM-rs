// Answer 0

#[test]
fn test_end_with_serialize_value_error() {
    struct MockMap {
        should_return_error: bool,
    }

    impl ser::SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

        fn serialize_value(&mut self, _: &Content) -> Result<(), Self::Error> {
            if self.should_return_error {
                Err(Error)
            } else {
                Ok(())
            }
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let name = "test";
    let fields = vec![Content::U8(42)];

    let result = {
        let mut map = MockMap { should_return_error: true };
        let mut serializer = SerializeTupleVariantAsMapValue {
            map,
            name,
            fields,
        };
        serializer.end()
    };

    assert!(result.is_err());
}

#[test]
fn test_end_with_successful_serialization() {
    struct MockMap {
        should_return_error: bool,
    }

    impl ser::SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

        fn serialize_value(&mut self, _: &Content) -> Result<(), Self::Error> {
            if self.should_return_error {
                Err(Error)
            } else {
                Ok(())
            }
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let name = "test";
    let fields = vec![Content::U8(42)];

    let result = {
        let mut map = MockMap { should_return_error: false };
        let mut serializer = SerializeTupleVariantAsMapValue {
            map,
            name,
            fields,
        };
        serializer.end()
    };

    assert!(result.is_ok());
}

