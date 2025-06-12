// Answer 0

#[test]
fn test_borrow_cow_bytes_str() {
    use serde::de::{self, Deserializer, Visitor};
    use std::borrow::Cow;

    struct TestDeserializer {
        value: &'static str,
    }

    impl Deserializer<'_> for TestDeserializer {
        type Error = de::value::BorrowedStr;

        fn deserialize_bytes<V>(self, _: V) -> Result<Vec<u8>, Self::Error>
        where
            V: Visitor<'_>,
        {
            Ok(self.value.as_bytes().to_vec())
        }
    }

    let deserializer = TestDeserializer { value: "hello" };
    let result: Result<Cow<[u8]>, _> = borrow_cow_bytes(deserializer);
    assert_eq!(result.unwrap(), Cow::Owned(b"hello".to_vec()));
}

#[test]
fn test_borrow_cow_bytes_borrowed_str() {
    use serde::de::{self, Deserializer, Visitor};
    use std::borrow::Cow;

    struct TestDeserializer<'a> {
        value: &'a str,
    }

    impl<'a> Deserializer<'a> for TestDeserializer<'a> {
        type Error = de::value::BorrowedStr;

        fn deserialize_bytes<V>(self, _: V) -> Result<Vec<u8>, Self::Error>
        where
            V: Visitor<'a>,
        {
            Ok(self.value.as_bytes().to_vec())
        }
    }

    let s = "world";
    let deserializer = TestDeserializer { value: s };
    let result: Result<Cow<[u8]>, _> = borrow_cow_bytes(deserializer);
    assert_eq!(result.unwrap(), Cow::Borrowed(s.as_bytes()));
}

#[test]
fn test_borrow_cow_bytes_owned_string() {
    use serde::de::{self, Deserializer, Visitor};
    use std::borrow::Cow;

    struct TestDeserializer {
        value: String,
    }

    impl Deserializer<'_> for TestDeserializer {
        type Error = de::value::BorrowedStr;

        fn deserialize_bytes<V>(self, _: V) -> Result<Vec<u8>, Self::Error>
        where
            V: Visitor<'_>,
        {
            Ok(self.value.into_bytes())
        }
    }

    let deserializer = TestDeserializer {
        value: String::from("test"),
    };
    let result: Result<Cow<[u8]>, _> = borrow_cow_bytes(deserializer);
    assert_eq!(result.unwrap(), Cow::Owned(b"test".to_vec()));
}

#[test]
fn test_borrow_cow_bytes_vec_u8() {
    use serde::de::{self, Deserializer, Visitor};
    use std::borrow::Cow;

    struct TestDeserializer {
        value: Vec<u8>,
    }

    impl Deserializer<'_> for TestDeserializer {
        type Error = de::value::BorrowedStr;

        fn deserialize_bytes<V>(self, _: V) -> Result<Vec<u8>, Self::Error>
        where
            V: Visitor<'_>,
        {
            Ok(self.value)
        }
    }

    let deserializer = TestDeserializer {
        value: b"vec_test".to_vec(),
    };
    let result: Result<Cow<[u8]>, _> = borrow_cow_bytes(deserializer);
    assert_eq!(result.unwrap(), Cow::Owned(b"vec_test".to_vec()));
}

#[test]
fn test_borrow_cow_bytes_borrowed_bytes() {
    use serde::de::{self, Deserializer, Visitor};
    use std::borrow::Cow;

    struct TestDeserializer<'a> {
        value: &'a [u8],
    }

    impl<'a> Deserializer<'a> for TestDeserializer<'a> {
        type Error = de::value::BorrowedStr;

        fn deserialize_bytes<V>(self, _: V) -> Result<Vec<u8>, Self::Error>
        where
            V: Visitor<'a>,
        {
            Ok(self.value.to_vec())
        }
    }

    let bytes: &[u8] = b"borrowed_bytes";
    let deserializer = TestDeserializer { value: bytes };
    let result: Result<Cow<[u8]>, _> = borrow_cow_bytes(deserializer);
    assert_eq!(result.unwrap(), Cow::Borrowed(bytes));
}

