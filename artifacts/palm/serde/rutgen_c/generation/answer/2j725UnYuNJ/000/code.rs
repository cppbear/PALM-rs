// Answer 0

#[test]
fn test_serialize_f64() {
    struct MockMap {
        error: Option<Error>,
    }

    impl SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

        fn serialize_key(&mut self, _: &str) -> Result<(), Self::Error> {
            Ok(())
        }

        fn serialize_entry(&mut self, _: &str, _: &()) -> Result<(), Self::Error> {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockMap { error: None };
    let serializer = FlatMapSerializer(&mut map);
    let result = serializer.serialize_f64(3.14);

    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.to_string(), "can only flatten structs and maps (got Float)");
    }
}

