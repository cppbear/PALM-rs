// Answer 0

#[test]
fn test_end_success() {
    struct MockMap {
        should_serialize: bool,
    }

    impl serde::SerializeMap for MockMap {
        type Ok = ();
        type Error = serde::Error;

        fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + serde::Serialize,
        {
            Ok(())
        }

        fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + serde::Serialize,
        {
            if self.should_serialize {
                Ok(())
            } else {
                Err(serde::Error)
            }
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut mock_map = MockMap {
        should_serialize: true,
    };

    let name = "test_struct";
    let fields = vec![
        ("field1", Content::U32(42)),
        ("field2", Content::String("value".to_string())),
    ];

    let serializer = FlatMapSerializeStructVariantAsMapValue {
        map: &mut mock_map,
        name,
        fields,
    };

    let result = serializer.end();
    assert_eq!(result, Ok(()));
}

