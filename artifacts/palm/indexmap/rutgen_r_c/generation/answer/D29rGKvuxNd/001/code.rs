// Answer 0

#[test]
fn test_first_mut_with_non_empty_index_map() {
    struct CustomIndexMap {
        core: IndexMapCore<i32, String>,
    }

    impl CustomIndexMap {
        fn new() -> Self {
            CustomIndexMap {
                core: IndexMapCore {
                    indices: Indices::new(),
                    entries: Entries::new(),
                }
            }
        }

        fn insert(&mut self, key: i32, value: String) {
            // Simulating insert of key-value pairs into the entries
            self.core.entries.push(Bucket {
                hash: HashValue::new(key),
                key,
                value,
            });
            // Update indices appropriately (not shown for brevity)
        }

        fn as_entries_mut(&mut self) -> &mut [Bucket<i32, String>] {
            &mut self.core.entries
        }
    }

    // Create and fill the CustomIndexMap
    let mut index_map = CustomIndexMap::new();
    index_map.insert(1, "one".to_string());
    index_map.insert(2, "two".to_string());

    // Check the first mutable entry
    let first_entry = index_map.first_mut();
    assert!(first_entry.is_some());
    let (key, value) = first_entry.unwrap();
    assert_eq!(*key, 1);
    *value = "modified".to_string();
    assert_eq!(index_map.core.entries[0].value, "modified");
}

#[test]
fn test_first_mut_with_empty_index_map() {
    struct CustomIndexMap {
        core: IndexMapCore<i32, String>,
    }

    impl CustomIndexMap {
        fn new() -> Self {
            CustomIndexMap {
                core: IndexMapCore {
                    indices: Indices::new(),
                    entries: Entries::new(),
                }
            }
        }
        
        fn as_entries_mut(&mut self) -> &mut [Bucket<i32, String>] {
            &mut self.core.entries
        }
    }

    // Create an empty CustomIndexMap
    let mut index_map = CustomIndexMap::new();

    // Check the first mutable entry
    let first_entry = index_map.first_mut();
    assert!(first_entry.is_none());
}

