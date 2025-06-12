// Answer 0

#[test]
fn test_end_with_serialize_value_error() {
    struct MockMap {
        should_fail: bool,
    }

    impl ser::SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

        fn serialize_value(&mut self, _value: &Content) -> Result<(), Self::Error> {
            if self.should_fail {
                Err(Error)
            } else {
                Ok(())
            }
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockMap { should_fail: true };
    let fields = vec![("field1", Content::U32(42))];
    let struct_variant = SerializeStructVariantAsMapValue {
        map,
        name: "TestStruct",
        fields,
    };

    let result = struct_variant.end();
    assert!(result.is_err());
}

