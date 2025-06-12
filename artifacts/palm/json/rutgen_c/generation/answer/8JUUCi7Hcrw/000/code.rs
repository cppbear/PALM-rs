// Answer 0

#[test]
fn test_to_vec_pretty_with_integer() {
    struct MyInteger(i32);

    impl Serialize for MyInteger {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            serializer.serialize_i32(self.0)
        }
    }

    let value = MyInteger(42);
    let result = to_vec_pretty(&value);
    assert!(result.is_ok());
    let json_bytes = result.unwrap();
    assert_eq!(json_bytes, b"42");
}

#[test]
fn test_to_vec_pretty_with_string() {
    struct MyString(String);

    impl Serialize for MyString {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            serializer.serialize_str(&self.0)
        }
    }

    let value = MyString("hello".to_string());
    let result = to_vec_pretty(&value);
    assert!(result.is_ok());
    let json_bytes = result.unwrap();
    assert_eq!(json_bytes, b"\"hello\"");
}

#[test]
fn test_to_vec_pretty_with_array() {
    struct MyArray(Vec<i32>);

    impl Serialize for MyArray {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
            for elem in &self.0 {
                seq.serialize_element(elem)?;
            }
            seq.end()
        }
    }

    let value = MyArray(vec![1, 2, 3]);
    let result = to_vec_pretty(&value);
    assert!(result.is_ok());
    let json_bytes = result.unwrap();
    assert_eq!(json_bytes, b"[1,2,3]");
}

#[test]
fn test_to_vec_pretty_with_invalid_map() {
    use serde::ser::Impossible;

    struct InvalidMap;

    impl Serialize for InvalidMap {
        fn serialize<S>(&self, _serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            Err(S::Error::custom("Non-string key"))
        }
    }

    let value = InvalidMap;
    let result = to_vec_pretty(&value);
    assert!(result.is_err());
}

#[test]
fn test_to_vec_pretty_with_empty_map() {
    use std::collections::HashMap;

    struct MyMap(HashMap<String, i32>);

    impl Serialize for MyMap {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            let mut map = serializer.serialize_map(Some(self.0.len()))?;
            for (key, value) in &self.0 {
                map.serialize_key(key)?;
                map.serialize_value(value)?;
            }
            map.end()
        }
    }

    let value = MyMap(HashMap::new());
    let result = to_vec_pretty(&value);
    assert!(result.is_ok());
    let json_bytes = result.unwrap();
    assert_eq!(json_bytes, b"{}");
}

