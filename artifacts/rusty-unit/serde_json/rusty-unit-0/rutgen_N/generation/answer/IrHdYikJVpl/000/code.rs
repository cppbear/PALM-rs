// Answer 0

#[test]
fn test_deserialize_bytes_success() {
    use serde::de::{self, Visitor};
    use serde_json::Error;
    use std::marker::PhantomData;

    struct TestVisitor {
        marker: PhantomData<u8>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a byte buffer")
        }

        fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(vec![1, 2, 3, 4])
        }
    }

    let input: &[u8] = &[1, 2, 3, 4];
    let visitor = TestVisitor { marker: PhantomData };
    let result: Result<Vec<u8>, Error> = input.deserialize_bytes(visitor);
    assert_eq!(result.unwrap(), vec![1, 2, 3, 4]);
}

#[test]
#[should_panic]
fn test_deserialize_bytes_invalid() {
    use serde::de::{self, Visitor};
    use serde_json::Error;
    use std::marker::PhantomData;

    struct InvalidVisitor {
        marker: PhantomData<u8>,
    }

    impl<'de> Visitor<'de> for InvalidVisitor {
        type Value = Vec<u8>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a byte buffer")
        }

        fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Err(de::Error::custom("Invalid byte data"))
        }
    }

    let input: &[u8] = &[1, 2, 3, 4];
    let visitor = InvalidVisitor { marker: PhantomData };
    let _result: Result<Vec<u8>, Error> = input.deserialize_bytes(visitor);
}

