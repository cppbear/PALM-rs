// Answer 0

#[test]
fn test_swap_take_element_present() {
    struct MockHashBuilder;

    impl BuildHasher for MockHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    let mut index_set = IndexSet::<i32, MockHashBuilder> {
        map: IndexMap::new(MockHashBuilder),
    };

    index_set.map.insert(1, ());
    index_set.map.insert(2, ());

    assert_eq!(index_set.swap_take(&1), Some(1));
    assert_eq!(index_set.swap_take(&2), Some(2));
}

#[test]
fn test_swap_take_element_absent() {
    struct MockHashBuilder;

    impl BuildHasher for MockHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    let mut index_set = IndexSet::<i32, MockHashBuilder> {
        map: IndexMap::new(MockHashBuilder),
    };

    index_set.map.insert(1, ());
    index_set.map.insert(2, ());

    assert_eq!(index_set.swap_take(&3), None);
}

#[test]
fn test_swap_take_empty_set() {
    struct MockHashBuilder;

    impl BuildHasher for MockHashBuilder {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }

    let mut index_set = IndexSet::<i32, MockHashBuilder> {
        map: IndexMap::new(MockHashBuilder),
    };

    assert_eq!(index_set.swap_take(&1), None);
}

