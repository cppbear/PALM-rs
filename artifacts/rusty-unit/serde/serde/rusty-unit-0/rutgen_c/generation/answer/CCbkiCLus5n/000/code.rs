// Answer 0

#[test]
fn test_serialize_key_success() {
    struct MockSerializeMap {
        called: bool,
    }

    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = Error;

        fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            self.called = true;
            Ok(())
        }

        fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut mock = MockSerializeMap { called: false };
    let mut flat_map = FlatMapSerializeMap(&mut mock);

    let result = flat_map.serialize_key(&"test_key");
    assert!(result.is_ok());
    assert!(mock.called);
}

#[test]
fn test_serialize_key_error() {
    struct MockSerializeMap {
        should_fail: bool,
    }

    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = Error;

        fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            if self.should_fail {
                Err(Error {})
            } else {
                Ok(())
            }
        }

        fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut mock = MockSerializeMap { should_fail: true };
    let mut flat_map = FlatMapSerializeMap(&mut mock);

    let result = flat_map.serialize_key(&"test_key");
    assert!(result.is_err());
}

