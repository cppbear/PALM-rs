// Answer 0

#[derive(Default)]
struct ContentSerializer<E> {
    entries: Vec<(String, String)>,
    _marker: std::marker::PhantomData<E>,
}

impl<E> ContentSerializer<E> {
    fn new() -> Self {
        Self::default()
    }
}

impl<E> ContentSerializer<E>
where
    E: std::fmt::Debug,
{
    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), E>
    where
        K: ?Sized + serde::ser::Serialize,
        V: ?Sized + serde::ser::Serialize,
    {
        let key = key.serialize(&mut *self)?;
        let value = value.serialize(&mut *self)?;
        self.entries.push((key, value));
        Ok(())
    }
}

#[derive(serde::Serialize)]
struct TestKey {
    id: i32,
}

#[derive(serde::Serialize)]
struct TestValue {
    name: String,
}

#[test]
fn test_serialize_entry_with_valid_data() {
    let mut serializer = ContentSerializer::<()>::new();
    let key = TestKey { id: 1 };
    let value = TestValue { name: "test".to_string() };
    
    let result = serializer.serialize_entry(&key, &value);
    
    assert!(result.is_ok());
    assert_eq!(serializer.entries.len(), 1);
}

#[test]
fn test_serialize_entry_with_empty_key_value() {
    let mut serializer = ContentSerializer::<()>::new();
    let key = TestKey { id: 0 };
    let value = TestValue { name: "".to_string() };
    
    let result = serializer.serialize_entry(&key, &value);
    
    assert!(result.is_ok());
    assert_eq!(serializer.entries.len(), 1);
}

