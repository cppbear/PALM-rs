// Answer 0

#[derive(serde::Serialize)]
struct SerializeKey {
    id: i32,
}

#[derive(serde::Serialize)]
struct SerializeValue {
    name: String,
}

struct ContentSerializer<E> {
    _marker: std::marker::PhantomData<E>,
}

impl<E> ContentSerializer<E> {
    fn new() -> Self {
        ContentSerializer {
            _marker: std::marker::PhantomData,
        }
    }
}

struct TestSerializer<E> {
    entries: Vec<(serde_json::Value, serde_json::Value)>,
    _marker: std::marker::PhantomData<E>,
}

impl<E> TestSerializer<E> {
    fn new() -> Self {
        TestSerializer {
            entries: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), E>
    where
        K: ?Sized + serde::Serialize,
        V: ?Sized + serde::Serialize,
    {
        let key = key.serialize(ContentSerializer::<E>::new()).map_err(|_| todo!())?;
        let value = value.serialize(ContentSerializer::<E>::new()).map_err(|_| todo!())?;
        self.entries.push((key, value));
        Ok(())
    }
}

#[test]
fn test_serialize_entry_success() {
    let mut serializer = TestSerializer::<()>::new();
    let key = SerializeKey { id: 1 };
    let value = SerializeValue { name: String::from("Test") };

    let result = serializer.serialize_entry(&key, &value);
    assert!(result.is_ok());
}

