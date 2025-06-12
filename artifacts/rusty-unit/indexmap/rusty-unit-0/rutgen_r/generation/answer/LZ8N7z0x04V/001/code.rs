// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap; // Assuming you're using IndexMap from indexmap crate

    #[test]
    fn test_index_existing_key() {
        let mut map: IndexMap<&str, i32> = IndexMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        let value = map.index("a");
        assert_eq!(*value, 1);
    }

    #[test]
    #[should_panic(expected = "no entry found for key")]
    fn test_index_non_existing_key() {
        let map: IndexMap<&str, i32> = IndexMap::new();
        map.index("non_existing_key"); // This should panic
    }

    #[test]
    fn test_index_multiple_entries() {
        let mut map: IndexMap<&str, i32> = IndexMap::new();
        map.insert("x", 10);
        map.insert("y", 20);
        map.insert("z", 30);

        let value1 = map.index("x");
        let value2 = map.index("y");
        let value3 = map.index("z");

        assert_eq!(*value1, 10);
        assert_eq!(*value2, 20);
        assert_eq!(*value3, 30);
    }

    #[test]
    #[should_panic(expected = "no entry found for key")]
    fn test_index_empty_map() {
        let map: IndexMap<&str, i32> = IndexMap::new();
        map.index("key"); // This should panic as the map is empty
    }
}

