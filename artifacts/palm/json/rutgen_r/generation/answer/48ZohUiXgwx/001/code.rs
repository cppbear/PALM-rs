// Answer 0

#[test]
fn test_or_insert_occupied_entry() {
    use serde_json::{json, Value, Map, Entry};

    struct TestMap {
        inner: Map<String, Value>,
    }

    impl TestMap {
        fn new() -> Self {
            Self {
                inner: Map::new(),
            }
        }

        fn entry(&mut self, key: &str) -> Entry<String, Value> {
            self.inner.entry(key.to_string())
        }
    }

    let mut test_map = TestMap::new();
    test_map.entry("key1").or_insert(json!(10)); // create an occupied entry

    // Test that we can mutate the value and ensure it is correctly referenced
    let value_ref = test_map.entry("key1").or_insert(json!(20));
    *value_ref = json!(15); // Modify the value

    assert_eq!(test_map.inner["key1"], 15);

    // Test inserting with a value that should not change the existing one
    let value_ref_second = test_map.entry("key1").or_insert(json!(25));
    assert_eq!(value_ref_second, &mut json!(15)); // should return mutable reference to existing value
    assert_eq!(test_map.inner["key1"], 15); // ensure value did not change
}

