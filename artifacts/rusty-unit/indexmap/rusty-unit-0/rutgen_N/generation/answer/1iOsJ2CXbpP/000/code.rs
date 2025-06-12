// Answer 0

#[test]
fn test_swap_remove_full_key_present() {
    use indexmap::IndexMap;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct MyHasher {
        hasher: DefaultHasher,
    }

    impl MyHasher {
        fn new() -> Self {
            MyHasher {
                hasher: DefaultHasher::new(),
            }
        }
    }

    impl Hasher for MyHasher {
        fn finish(&self) -> u64 {
            self.hasher.finish()
        }

        fn write(&mut self, bytes: &[u8]) {
            self.hasher.write(bytes);
        }
    }

    let mut map: IndexMap<&str, i32> = IndexMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    let result = map.swap_remove_full("b");
    assert_eq!(result, Some((1, "b", 2)));
}

#[test]
fn test_swap_remove_full_key_absent() {
    use indexmap::IndexMap;

    let mut map: IndexMap<&str, i32> = IndexMap::new();
    map.insert("a", 1);
    map.insert("b", 2);

    let result = map.swap_remove_full("c");
    assert_eq!(result, None);
}

#[test]
fn test_swap_remove_full_last_element() {
    use indexmap::IndexMap;

    let mut map: IndexMap<&str, i32> = IndexMap::new();
    map.insert("a", 1);
    map.insert("b", 2);

    let result = map.swap_remove_full("b");
    assert_eq!(result, Some((1, "b", 2))); // Should return the last element
}

#[test]
fn test_swap_remove_full_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<&str, i32> = IndexMap::new();

    let result = map.swap_remove_full("a");
    assert_eq!(result, None); // Should return None for empty map
}

