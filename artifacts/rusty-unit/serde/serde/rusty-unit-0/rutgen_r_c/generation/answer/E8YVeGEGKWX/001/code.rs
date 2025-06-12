// Answer 0

#[test]
fn test_serialize_entry_key_error() {
    struct DummyError;
    impl std::fmt::Debug for DummyError {
        fn fmt(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }
    }
    impl ser::Error for DummyError {}

    struct FailingKey;
    impl Serialize for FailingKey {
        fn serialize<S>(&self, _: S) -> Result<(), S::Error>
        where
            S: Serializer,
        {
            Err(DummyError)
        }
    }

    struct ValidValue;
    impl Serialize for ValidValue {
        fn serialize<S>(&self, _: S) -> Result<(), S::Error>
        where
            S: Serializer,
        {
            Ok(())
        }
    }

    let mut map = SerializeMap::<DummyError> {
        entries: vec![],
        key: None,
        error: PhantomData,
    };

    let result = map.serialize_entry(&FailingKey, &ValidValue);
    assert!(result.is_err());
}

#[test]
fn test_serialize_entry_value_error() {
    struct DummyError;
    impl std::fmt::Debug for DummyError {
        fn fmt(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }
    }
    impl ser::Error for DummyError {}

    struct ValidKey;
    impl Serialize for ValidKey {
        fn serialize<S>(&self, _: S) -> Result<(), S::Error>
        where
            S: Serializer,
        {
            Ok(())
        }
    }

    struct FailingValue;
    impl Serialize for FailingValue {
        fn serialize<S>(&self, _: S) -> Result<(), S::Error>
        where
            S: Serializer,
        {
            Err(DummyError)
        }
    }

    let mut map = SerializeMap::<DummyError> {
        entries: vec![],
        key: None,
        error: PhantomData,
    };

    let result = map.serialize_entry(&ValidKey, &FailingValue);
    assert!(result.is_err());
}

