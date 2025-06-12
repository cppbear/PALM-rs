// Answer 0

#[test]
fn test_get_index_of_existing_item() {
    use indexmap::IndexSet;
    use std::hash::Hash;

    struct CustomItem {
        id: i32,
    }

    impl Hash for CustomItem {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.id.hash(state);
        }
    }

    impl PartialEq for CustomItem {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    let mut set = IndexSet::new();
    set.insert(CustomItem { id: 1 });
    set.insert(CustomItem { id: 2 });

    let index = set.get_index_of(&CustomItem { id: 1 });
    assert_eq!(index, Some(0));
}

#[test]
fn test_get_index_of_non_existing_item() {
    use indexmap::IndexSet;
    use std::hash::Hash;

    struct CustomItem {
        id: i32,
    }

    impl Hash for CustomItem {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.id.hash(state);
        }
    }

    impl PartialEq for CustomItem {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    let mut set = IndexSet::new();
    set.insert(CustomItem { id: 1 });
    set.insert(CustomItem { id: 2 });

    let index = set.get_index_of(&CustomItem { id: 3 });
    assert_eq!(index, None);
}

#[test]
fn test_get_index_of_empty_set() {
    use indexmap::IndexSet;
    use std::hash::Hash;

    struct CustomItem {
        id: i32,
    }

    impl Hash for CustomItem {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.id.hash(state);
        }
    }

    impl PartialEq for CustomItem {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    let set: IndexSet<CustomItem> = IndexSet::new();

    let index = set.get_index_of(&CustomItem { id: 1 });
    assert_eq!(index, None);
} 

#[test]
#[should_panic]
fn test_get_index_of_with_unhashable_item() {
    use indexmap::IndexSet;

    struct UnhashableItem;

    let mut set = IndexSet::new();
    set.insert(UnhashableItem);

    let _ = set.get_index_of(&UnhashableItem);
}

