// Answer 0

#[test]
fn test_to_string_pretty_error() {
    struct InvalidSerialize;

    impl Serialize for InvalidSerialize {
        fn serialize<S>(&self, _serializer: S) -> Result<(), S::Error>
        where
            S: serde::ser::Serializer,
        {
            Err(S::Error::custom("serialization error"))
        }
    }

    let invalid_value = InvalidSerialize;
    let result: Result<String> = to_string_pretty(&invalid_value);
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.to_string(), "serialization error");
    }
}

