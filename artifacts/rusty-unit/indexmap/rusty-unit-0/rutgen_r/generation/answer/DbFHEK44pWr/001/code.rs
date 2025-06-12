// Answer 0

#[derive(Debug)]
struct Key {
    value: i32,
}

#[derive(Debug)]
struct Entry<K, V> {
    key: K,
    value: Option<V>,
}

impl<K, V> Entry<K, V> {
    fn occupied(key: K, value: V) -> Self {
        Entry {
            key,
            value: Some(value),
        }
    }

    fn vacant(key: K) -> Self {
        Entry {
            key,
            value: None,
        }
    }

    fn into_mut(&mut self) -> &mut V {
        self.value.as_mut().unwrap() // safe as it's occupied
    }

    fn insert(&mut self, value: V) -> &mut V {
        self.value.get_or_insert(value) // inserts and returns a mutable reference
    }
}

#[derive(Debug)]
enum EntryType<K, V> {
    Occupied(Entry<K, V>),
    Vacant(Entry<K, V>),
}

impl<K: Copy, V> EntryType<K, V> {
    pub fn or_insert_with_key<F>(self, call: F) -> &'_ mut V
    where
        F: FnOnce(&K) -> V,
    {
        match self {
            EntryType::Occupied(entry) => entry.into_mut(),
            EntryType::Vacant(mut entry) => {
                let value = call(&entry.key);
                entry.insert(value)
            }
        }
    }
}

#[test]
fn test_or_insert_with_key_vacant_case() {
    let key = Key { value: 42 };
    let entry = EntryType::Vacant(entry(key));
    
    let mutable_value = entry.or_insert_with_key(|k| {
        assert_eq!(k.value, 42);
        100 // return some value
    });

    assert_eq!(*mutable_value, 100);
}

#[test]
fn test_or_insert_with_key_occupied_case() {
    let key = Key { value: 42 };
    let mut entry = EntryType::Occupied(Entry::occupied(key, 200));
    
    let mutable_value = entry.or_insert_with_key(|k| {
        panic!("This should not be called since the entry is occupied")
    });

    assert_eq!(*mutable_value, 200);
}

#[test]
#[should_panic] 
fn test_or_insert_with_key_panic_on_occupied() {
    let key = Key { value: 42 };
    let entry = EntryType::Occupied(Entry::occupied(key, 200));

    let _ = entry.or_insert_with_key(|_k| {
        panic!("This should panic since the entry is occupied")
    });
}

fn entry(key: Key) -> Entry<Key, i32> {
    Entry::vacant(key)
}

