// Answer 0

#[test]
fn test_into_mut() {
    // Define a simple representation for Entries and Indices for testing purposes
    struct SimpleEntries<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    struct SimpleIndices {
        len: usize,
    }

    // Mockup of the IndexMapCore to provide necessary context for our tests
    struct SimpleIndexMapCore<K, V> {
        indices: SimpleIndices,
        entries: SimpleEntries<K, V>,
    }

    // Create instances of K and V types
    struct Key(usize);
    struct Value(usize);

    // Initialize data structures
    let mut map = SimpleIndexMapCore {
        indices: SimpleIndices { len: 1 },
        entries: SimpleEntries {
            entries: vec![Bucket {
                hash: HashValue::default(),
                key: Key(0),
                value: Value(42),
            }],
        },
    };

    let index = 0;

    // Create IndexedEntry instance
    let mut entry = IndexedEntry::new(&mut map, index);

    // Use the into_mut method to get a mutable reference to the value
    let value_ref: &mut Value = entry.into_mut();

    // Modify the value through the mutable reference
    value_ref.0 = 100;

    // Ensure the original map reflects the modification
    assert_eq!(map.entries.entries[0].value.0, 100);
}

