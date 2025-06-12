// Answer 0

#[derive(Debug)]
struct TestMapAccess {
    keys: Vec<String>,
    index: usize,
}

impl TestMapAccess {
    fn new(keys: Vec<String>) -> Self {
        Self { keys, index: 0 }
    }

    fn next_key<K: serde::de::Deserialize<'static>>(&mut self) -> Result<Option<K>, serde::de::value::Error> {
        if self.index < self.keys.len() {
            let key = self.keys[self.index].clone();
            self.index += 1;
            Ok(Some(serde::de::value::from_str::<K>(&key).map_err(|_| serde::de::value::Error::custom("Invalid key"))?))
        } else {
            Ok(None)
        }
    }
}

#[test]
fn test_next_key_some() {
    let mut access = TestMapAccess::new(vec!["key1".into(), "key2".into()]);
    
    let result: Result<Option<String>, _> = access.next_key();
    assert_eq!(result.unwrap(), Some("key1".to_string()));
    
    let result: Result<Option<String>, _> = access.next_key();
    assert_eq!(result.unwrap(), Some("key2".to_string()));
}

#[test]
fn test_next_key_none() {
    let mut access = TestMapAccess::new(vec![]);
    
    let result: Result<Option<String>, _> = access.next_key();
    assert_eq!(result.unwrap(), None);
}

#[test]
fn test_next_key_boundary() {
    let mut access = TestMapAccess::new(vec!["key1".into()]);
    
    let result: Result<Option<String>, _> = access.next_key();
    assert_eq!(result.unwrap(), Some("key1".to_string()));
    
    let result: Result<Option<String>, _> = access.next_key();
    assert_eq!(result.unwrap(), None);
}

