// Answer 0

#[test]
fn test_serialize_key_with_string() {
    use serde::ser::{Serializer, Serialize};
    use core::result::Result;

    struct TestKey<'a>(&'a str);

    impl Serialize for TestKey<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(self.0)
        }
    }

    let mut map = SerializeMap::Map {
        map: Map::new(),
        next_key: None,
    };
    let key = TestKey("test_key");

    assert_eq!(map.serialize_key(&key), Ok(()));
}

#[test]
fn test_serialize_key_with_empty_string() {
    use serde::ser::{Serializer, Serialize};
    use core::result::Result;

    struct TestKey<'a>(&'a str);

    impl Serialize for TestKey<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(self.0)
        }
    }

    let mut map = SerializeMap::Map {
        map: Map::new(),
        next_key: None,
    };
    let key = TestKey("");

    assert_eq!(map.serialize_key(&key), Ok(()));
}

#[test]
fn test_serialize_key_with_numeric_string() {
    use serde::ser::{Serializer, Serialize};
    use core::result::Result;

    struct TestKey<'a>(&'a str);

    impl Serialize for TestKey<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(self.0)
        }
    }

    let mut map = SerializeMap::Map {
        map: Map::new(),
        next_key: None,
    };
    let key = TestKey("12345");

    assert_eq!(map.serialize_key(&key), Ok(()));
}

