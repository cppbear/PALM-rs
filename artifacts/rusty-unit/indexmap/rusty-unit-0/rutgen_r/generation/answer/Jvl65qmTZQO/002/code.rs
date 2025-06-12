// Answer 0

#[test]
fn test_swap_remove_entry_key_not_found() {
    use indexmap::IndexMap;
    use std::hash::{Hash, Hasher};

    struct MyKey {
        val: i32,
    }

    impl PartialEq for MyKey {
        fn eq(&self, other: &Self) -> bool {
            self.val == other.val
        }
    }

    impl Hash for MyKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.val.hash(state);
        }
    }

    struct MyEquivalent;

    impl<K> Equivalent<K> for MyEquivalent {
        fn equivalent(&self, _: &K) -> bool {
            false // This ensures no key will match, simulating a key not found scenario
        }
    }

    let mut map = IndexMap::new();
    map.insert(MyKey { val: 1 }, "value1");
    map.insert(MyKey { val: 2 }, "value2");

    let key_to_remove = MyKey { val: 3 }; // This key does not exist in the map
    let result = map.swap_remove_entry(&key_to_remove);
    
    assert_eq!(result, None); // Expecting None since the key does not exist
}

