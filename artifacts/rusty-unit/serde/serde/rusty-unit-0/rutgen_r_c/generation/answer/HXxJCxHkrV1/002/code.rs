// Answer 0

#[test]
fn test_end_success() {
    struct MockMap {
        should_succeed: bool,
    }

    impl SerializeMap for MockMap {
        type Error = Error;

        fn serialize_value(&mut self, _value: &Content) -> Result<(), Self::Error> {
            if self.should_succeed {
                Ok(())
            } else {
                Err(Error { err: ErrorImpl })
            }
        }
    }

    let mut map = MockMap { should_succeed: true };
    let mut fields = vec![Content::U32(1), Content::U32(2)];

    let serializer = FlatMapSerializeTupleVariantAsMapValue {
        map: &mut map,
        fields,
    };

    let result = serializer.end();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_end_panic_on_serialize_value_fail() {
    struct MockFailingMap;

    impl SerializeMap for MockFailingMap {
        type Error = Error;

        fn serialize_value(&mut self, _value: &Content) -> Result<(), Self::Error> {
            Err(Error { err: ErrorImpl })
        }
    }

    let mut map = MockFailingMap;
    let fields = vec![Content::U32(1), Content::U32(2)];

    let serializer = FlatMapSerializeTupleVariantAsMapValue {
        map: &mut map,
        fields,
    };

    let _ = serializer.end(); // This should panic
}

