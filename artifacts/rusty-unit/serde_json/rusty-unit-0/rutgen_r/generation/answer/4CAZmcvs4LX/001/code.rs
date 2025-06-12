// Answer 0

#[derive(Default)]
struct Serializer {
    vec: Vec<serde_json::Value>,
}

impl Serializer {
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + serde::ser::Serialize,
    {
        self.vec.push(serde_json::to_value(value)?);
        Ok(())
    }
}

#[test]
fn test_serialize_element_with_err() {
    struct ErroneousType;

    impl serde::ser::Serialize for ErroneousType {
        fn serialize<S>(&self, _serializer: S) -> Result<(), S::Error>
        where
            S: serde::Serializer,
        {
            Err(serde::ser::Error::custom("Serialization error"))
        }
    }

    let mut serializer = Serializer::default();
    let result = serializer.serialize_element(&ErroneousType);
    
    match result {
        Err(err) => assert_eq!(err.to_string(), "Serialization error"),
        _ => panic!("Expected an error but got a successful result"),
    }
}

