// Answer 0

fn test_end_success() -> Result<(), Error> {
    struct MockMap {
        serialized: bool,
    }

    impl ser::SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

        fn serialize_value(&mut self, _value: &Content) -> Result<(), Self::Error> {
            self.serialized = true;
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            if self.serialized {
                Ok(())
            } else {
                Err(Error)
            }
        }
    }

    let mut map = MockMap { serialized: false };
    let struct_variant = SerializeStructVariantAsMapValue {
        map,
        name: "test",
        fields: vec![("field1", Content::U32(42))],
    };

    let result = struct_variant.end();
    assert!(result.is_ok());
    Ok(())
}

#[test]
fn test_end_panic_condition() {
    struct MockMap {
        serialized: bool,
        end_called: bool,
    }

    impl ser::SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

        fn serialize_value(&mut self, _value: &Content) -> Result<(), Self::Error> {
            self.serialized = false; // Triggers panic condition
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            assert!(!self.serialized, "Expected serialize_value to be called successfully.");
            self.end_called = true;
            Ok(())
        }
    }

    let map = MockMap { serialized: false, end_called: false };
    let struct_variant = SerializeStructVariantAsMapValue {
        map,
        name: "test",
        fields: vec![("field1", Content::U32(42))],
    };

    let result = struct_variant.end();
    assert!(result.is_err());
}

