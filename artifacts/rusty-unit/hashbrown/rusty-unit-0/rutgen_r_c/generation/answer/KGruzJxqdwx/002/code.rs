// Answer 0

#[test]
fn test_entry_occupied_debug() {
    use hashbrown::{HashTable, DefaultHashBuilder, Entry, OccupiedEntry};

    // Initialize a HashTable and a hasher
    let mut table: HashTable<&str, i32> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let key = "key";
    let value = 42;

    // Insert an entry to create an OccupiedEntry
    table.insert(key, value);

    // Retrieve the occupied entry
    let entry = table.entry(key, |&x| x == key, |val| hasher.hash_one(val));

    if let Entry::Occupied(occupied_entry) = entry {
        // Test the formatting of the occupied entry
        let mut output = String::new();
        let result = occupied_entry.fmt(&mut fmt::Formatter::new(&mut output));

        // Assert that formatting did not result in an error
        assert!(result.is_ok());
        // Check that output contains expected information
        assert!(output.contains("Entry"));
        assert!(output.contains(key));
        assert!(output.contains(&value.to_string()));
    } else {
        panic!("Expected an occupied entry but got a vacant one.");
    }
}

