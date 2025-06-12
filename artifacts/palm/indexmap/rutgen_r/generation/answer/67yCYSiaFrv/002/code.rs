// Answer 0

#[test]
fn test_get_full_existing_key() {
    use std::collections::HashSet;
    use std::hash::Hash;

    struct Entry<K, V> {
        key: K,
        value: V,
    }

    struct MyMap<K, V> {
        entries: Vec<Entry<K, V>>,
    }

    impl<K, V> MyMap<K, V>
    where
        K: Hash + Eq,
    {
        fn new() -> Self {
            MyMap {
                entries: Vec::new(),
            }
        }

        fn insert(&mut self, key: K, value: V) {
            self.entries.push(Entry { key, value });
        }

        fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
        where
            Q: Hash + Eq + ?Sized,
            K: Equivalent<Q>,
        {
            self.entries.iter().position(|entry| entry.key.equiv(key))
        }

        fn as_entries(&self) -> &[Entry<K, V>] {
            &self.entries
        }

        fn get_full<Q>(&self, key: &Q) -> Option<(usize, &K, &V)>
        where
            Q: ?Sized + Hash + Equivalent<K>,
        {
            if let Some(i) = self.get_index_of(key) {
                let entry = &self.as_entries()[i];
                Some((i, &entry.key, &entry.value))
            } else {
                None
            }
        }
    }

    trait Equivalent<Q> {
        fn equiv(&self, other: &Q) -> bool;
    }

    impl Equivalent<str> for String {
        fn equiv(&self, other: &str) -> bool {
            self == other
        }
    }

    let mut my_map = MyMap::new();
    my_map.insert(String::from("key1"), 10);
    my_map.insert(String::from("key2"), 20);

    let result = my_map.get_full("key1");
    assert_eq!(result, Some((0, &String::from("key1"), &10)));
}

#[test]
fn test_get_full_non_existing_key() {
    use std::collections::HashSet;
    use std::hash::Hash;

    struct Entry<K, V> {
        key: K,
        value: V,
    }

    struct MyMap<K, V> {
        entries: Vec<Entry<K, V>>,
    }

    impl<K, V> MyMap<K, V>
    where
        K: Hash + Eq,
    {
        fn new() -> Self {
            MyMap {
                entries: Vec::new(),
            }
        }

        fn insert(&mut self, key: K, value: V) {
            self.entries.push(Entry { key, value });
        }

        fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
        where
            Q: Hash + Eq + ?Sized,
            K: Equivalent<Q>,
        {
            self.entries.iter().position(|entry| entry.key.equiv(key))
        }

        fn as_entries(&self) -> &[Entry<K, V>] {
            &self.entries
        }

        fn get_full<Q>(&self, key: &Q) -> Option<(usize, &K, &V)>
        where
            Q: ?Sized + Hash + Equivalent<K>,
        {
            if let Some(i) = self.get_index_of(key) {
                let entry = &self.as_entries()[i];
                Some((i, &entry.key, &entry.value))
            } else {
                None
            }
        }
    }

    trait Equivalent<Q> {
        fn equiv(&self, other: &Q) -> bool;
    }

    impl Equivalent<str> for String {
        fn equiv(&self, other: &str) -> bool {
            self == other
        }
    }

    let mut my_map = MyMap::new();
    my_map.insert(String::from("key1"), 10);
    my_map.insert(String::from("key2"), 20);

    let result = my_map.get_full("key3");
    assert_eq!(result, None);
}

