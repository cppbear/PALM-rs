// Answer 0

#[test]
fn test_serialize_empty_map() {
    use serde_json::json;
    use serde::ser::Serializer;
    use serde::ser::SerializeMap;

    struct TestMap<'a> {
        data: &'a Vec<(&'a str, &'a str)>,
    }

    impl<'a> serde::ser::Serialize for TestMap<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            use serde::ser::SerializeMap;
            let mut map = serializer.serialize_map(Some(self.data.len()))?;
            for (k, v) in self.data {
                map.serialize_entry(*k, *v)?;
            }
            map.end()
        }
    }

    let test_map = TestMap { data: &Vec::new() };
    let result = serde_json::to_string(&test_map);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_entry_error() {
    use serde_json::json;
    use serde::ser::{Serializer, SerializeMap};

    struct TestMap<'a> {
        data: &'a Vec<(&'a str, Option<&'a str>)>,
    }

    impl<'a> serde::ser::Serialize for TestMap<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut map = serializer.serialize_map(Some(self.data.len()))?;
            for (k, v) in self.data {
                match v {
                    Some(val) => map.serialize_entry(*k, *val)?,
                    None => return Err(serde::ser::Error::custom("serialize_entry should not be None")),
                }
            }
            map.end()
        }
    }

    let test_map = TestMap { 
        data: &vec![("key1", Some("value1")), ("key2", None)] 
    };
    let _ = serde_json::to_string(&test_map);  // This should panic due to None value
}

