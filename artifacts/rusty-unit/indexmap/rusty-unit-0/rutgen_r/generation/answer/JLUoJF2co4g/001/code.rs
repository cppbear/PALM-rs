// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;
    use std::hash::{Hash, Hasher};

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct TestItem {
        id: usize,
        value: String,
    }

    impl Hash for TestItem {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.id.hash(state);
        }
    }

    #[test]
    fn test_get_full_existing_item() {
        let mut map: IndexMap<TestItem, ()> = IndexMap::new();
        let item = TestItem { id: 1, value: "one".to_string() };
        map.insert(item.clone(), ());

        let result = map.get_full(&item);
        assert_eq!(result, Some((0, &item)));
    }

    #[test]
    fn test_get_full_non_existing_item() {
        let mut map: IndexMap<TestItem, ()> = IndexMap::new();
        let item1 = TestItem { id: 1, value: "one".to_string() };
        let item2 = TestItem { id: 2, value: "two".to_string() };
        map.insert(item1.clone(), ());

        let result = map.get_full(&item2);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_full_multiple_items() {
        let mut map: IndexMap<TestItem, ()> = IndexMap::new();
        let item1 = TestItem { id: 1, value: "one".to_string() };
        let item2 = TestItem { id: 2, value: "two".to_string() };
        map.insert(item1.clone(), ());
        map.insert(item2.clone(), ());

        let result = map.get_full(&item1);
        assert_eq!(result, Some((0, &item1)));

        let result = map.get_full(&item2);
        assert_eq!(result, Some((1, &item2)));
    }

    #[test]
    #[should_panic]
    fn test_get_full_panic_due_to_misuse() {
        // This test is to intentionally trigger a panic condition, e.g., using an incorrect type.
        let mut map: IndexMap<TestItem, ()> = IndexMap::new();
        let item1 = TestItem { id: 1, value: "one".to_string() };
        map.insert(item1.clone(), ());

        // Trying to get full with a non-existing reference
        let invalid_query = "not a TestItem";
        let _result = map.get_full(&invalid_query); // This will cause a panic or error
    }
}

