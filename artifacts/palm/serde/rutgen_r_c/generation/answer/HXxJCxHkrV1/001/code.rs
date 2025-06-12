// Answer 0

#[test]
fn test_end_should_return_error_when_map_serialization_fails() {
    struct MockMap {
        should_fail: bool,
    }

    impl SerializeMap for MockMap {
        type Error = Error;

        fn serialize_value(&mut self, _value: &Content) -> Result<(), Self::Error> {
            if self.should_fail {
                Err(Error)
            } else {
                Ok(())
            }
        }

        // Required trait method implementations
        fn serialize_entry(&mut self, _key: &Content, _value: &Content) -> Result<(), Self::Error> {
            Ok(())
        }
        
        fn end(self) -> Result<(), Self::Error> {
            Ok(())
        }
    }
    
    let mut map = MockMap { should_fail: true };
    let mut fields = Vec::new();
    fields.push(Content::Bool(true));
    
    let serializer = FlatMapSerializeTupleVariantAsMapValue {
        map: &mut map,
        fields,
    };

    let result = serializer.end();
    assert!(result.is_err());
}

#[test]
fn test_end_should_return_ok_when_map_serialization_succeeds() {
    struct MockMap {
        should_fail: bool,
    }

    impl SerializeMap for MockMap {
        type Error = Error;

        fn serialize_value(&mut self, _value: &Content) -> Result<(), Self::Error> {
            if self.should_fail {
                Err(Error)
            } else {
                Ok(())
            }
        }

        // Required trait method implementations
        fn serialize_entry(&mut self, _key: &Content, _value: &Content) -> Result<(), Self::Error> {
            Ok(())
        }
        
        fn end(self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let mut map = MockMap { should_fail: false };
    let mut fields = Vec::new();
    fields.push(Content::Bool(true));
    
    let serializer = FlatMapSerializeTupleVariantAsMapValue {
        map: &mut map,
        fields,
    };

    let result = serializer.end();
    assert!(result.is_ok());
}

