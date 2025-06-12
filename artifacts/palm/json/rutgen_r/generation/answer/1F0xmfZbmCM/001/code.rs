// Answer 0

#[test]
fn test_visit_array_ref_with_err() {
    use serde::de::{self, Visitor, Deserializer};
    use serde_json::Value;
    use serde_json::Error;
    use std::marker::PhantomData;

    struct ErrVisitor {
        phantom: PhantomData<()>,
    }

    impl<'de> Visitor<'de> for ErrVisitor {
        type Value = ();

        fn visit_seq<T>(self, _: &mut T) -> Result<Self::Value, Error>
        where
            T: Deserializer<'de>,
        {
            Err(de::Error::custom("error during visit_seq"))
        }
    }

    let array: &[Value] = &[Value::Null, Value::Bool(true)];
    let visitor = ErrVisitor { phantom: PhantomData };

    let result = visit_array_ref(array, visitor);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "error during visit_seq");
}

