// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::{IndexMap, indexmap};

    #[test]
    fn test_swap_remove_entry() {
        let mut map: IndexMap<i32, &str> = indexmap! {
            1 => "one",
            2 => "two",
            3 => "three",
        };

        // Assuming we have a way to create an entry to swap remove
        let entry = map.get_index(1).unwrap(); // Get the entry of the key 2
        let (key, value) = map.swap_remove_entry(entry); // Perform the swap_remove_entry

        assert_eq!(key, 2);
        assert_eq!(value, "two");
        assert!(!map.contains_key(&2)); // Check that the key 2 has been removed
    }

    #[test]
    fn test_swap_remove_entry_last_element() {
        let mut map: IndexMap<i32, &str> = indexmap! {
            1 => "one",
            2 => "two",
        };

        let entry = map.get_index(1).unwrap(); // Get the entry of the first element
        let (key, value) = map.swap_remove_entry(entry); // Perform the swap_remove_entry

        assert_eq!(key, 1);
        assert_eq!(value, "one");
        assert!(!map.contains_key(&1)); // Check that the key 1 has been removed
        assert_eq!(map.len(), 1); // Ensure the map has one element left
    }

    #[test]
    fn test_swap_remove_entry_empty_map() {
        let mut map: IndexMap<i32, &str> = IndexMap::new();

        // Panic expected when trying to swap remove from an empty map
        let result = std::panic::catch_unwind(|| {
            let entry = map.get_index(0).unwrap(); // This will panic because the map is empty
            map.swap_remove_entry(entry);
        });

        assert!(result.is_err()); // Ensure that the panic occurred
    }
}

