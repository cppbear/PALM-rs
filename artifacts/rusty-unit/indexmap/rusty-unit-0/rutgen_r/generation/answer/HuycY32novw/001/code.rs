// Answer 0

#[test]
fn test_insert_full_new_item() {
    struct MockSet<T> {
        map: indexmap::IndexMap<T, ()>,
    }

    impl<T: Eq + std::hash::Hash> MockSet<T> {
        pub fn insert_full(&mut self, value: T) -> (usize, bool) {
            let (index, existing) = self.map.insert_full(value, ());
            (index, existing.is_none())
        }
    }

    let mut set = MockSet { map: indexmap::IndexMap::new() };
    let (index, is_new) = set.insert_full("a");
    assert_eq!(index, 0);
    assert!(is_new);
}

#[test]
fn test_insert_full_existing_item() {
    struct MockSet<T> {
        map: indexmap::IndexMap<T, ()>,
    }

    impl<T: Eq + std::hash::Hash> MockSet<T> {
        pub fn insert_full(&mut self, value: T) -> (usize, bool) {
            let (index, existing) = self.map.insert_full(value, ());
            (index, existing.is_none())
        }
    }

    let mut set = MockSet { map: indexmap::IndexMap::new() };
    let _ = set.insert_full("a");
    let (index, is_new) = set.insert_full("a");
    assert_eq!(index, 0);
    assert!(!is_new);
}

#[test]
fn test_insert_full_multiple_items() {
    struct MockSet<T> {
        map: indexmap::IndexMap<T, ()>,
    }

    impl<T: Eq + std::hash::Hash> MockSet<T> {
        pub fn insert_full(&mut self, value: T) -> (usize, bool) {
            let (index, existing) = self.map.insert_full(value, ());
            (index, existing.is_none())
        }
    }

    let mut set = MockSet { map: indexmap::IndexMap::new() };
    for i in 0..5 {
        let (index, is_new) = set.insert_full(i);
        assert_eq!(index, i);
        assert!(is_new);
    }
}

#[test]
fn test_insert_full_overwrite_check() {
    struct MockSet<T> {
        map: indexmap::IndexMap<T, ()>,
    }

    impl<T: Eq + std::hash::Hash> MockSet<T> {
        pub fn insert_full(&mut self, value: T) -> (usize, bool) {
            let (index, existing) = self.map.insert_full(value, ());
            (index, existing.is_none())
        }
    }

    let mut set = MockSet { map: indexmap::IndexMap::new() };
    let _ = set.insert_full("test");
    let (index, is_new) = set.insert_full("test");
    assert_eq!(index, 0);
    assert!(!is_new);
}

