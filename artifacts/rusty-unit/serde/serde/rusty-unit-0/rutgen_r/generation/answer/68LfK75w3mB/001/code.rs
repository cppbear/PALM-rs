// Answer 0

#[test]
fn test_next_key_some() {
    use serde::de::{Deserialize, MapAccess, Visitor};
    use serde::Deserializer;

    struct TestMapAccess {
        keys: Vec<String>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = serde::de::Error;

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<String>, Self::Error>
        where
            K: Deserialize<'de>,
        {
            if self.index < self.keys.len() {
                let key = self.keys[self.index].clone();
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }
    }

    let mut access = TestMapAccess {
        keys: vec!["key1".to_string(), "key2".to_string()],
        index: 0,
    };

    assert_eq!(access.next_key::<String>().unwrap(), Some("key1".to_string()));
    assert_eq!(access.next_key::<String>().unwrap(), Some("key2".to_string()));
    assert_eq!(access.next_key::<String>().unwrap(), None);
}

#[test]
#[should_panic]
fn test_next_key_panic() {
    use serde::de::{Deserialize, MapAccess, Visitor};
    use serde::Deserializer;

    struct PanicMapAccess {
        cause_panic: bool,
    }

    impl<'de> MapAccess<'de> for PanicMapAccess {
        type Error = serde::de::Error;

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<String>, Self::Error>
        where
            K: Deserialize<'de>,
        {
            if self.cause_panic {
                panic!("Intentional panic for testing");
            }
            Ok(Some("key".to_string()))
        }
    }

    let mut access = PanicMapAccess { cause_panic: true };
    let _ = access.next_key::<String>().unwrap();
}

