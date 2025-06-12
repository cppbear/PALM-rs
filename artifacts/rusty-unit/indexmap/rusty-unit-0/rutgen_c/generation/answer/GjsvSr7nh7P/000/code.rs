// Answer 0

#[test]
fn test_shift_insert_new_value() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut set: crate::IndexSet<char, TestHasher> = ('a'..='z').collect();
    assert_eq!(set.get_index_of(&'*'), None);
    assert_eq!(set.shift_insert(10, '*'), true);
    assert_eq!(set.get_index_of(&'*'), Some(10));
}

#[test]
fn test_shift_insert_move_existing_value() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut set: crate::IndexSet<char, TestHasher> = ('a'..='z').collect();
    assert_eq!(set.shift_insert(10, 'a'), false);
    assert_eq!(set.get_index_of(&'a'), Some(10));
}

#[test]
fn test_shift_insert_move_existing_value_shifted() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut set: crate::IndexSet<char, TestHasher> = ('a'..='z').collect();
    assert_eq!(set.shift_insert(9, 'z'), false);
    assert_eq!(set.get_index_of(&'z'), Some(9));
}

#[test]
fn test_shift_insert_invalid_move_index() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut set: crate::IndexSet<char, TestHasher> = ('a'..='z').collect();
    let len = set.len();
    assert_eq!(set.shift_insert(len - 1, '*'), false);
    assert_eq!(set.get_index_of(&'*'), Some(len));
}

#[test]
#[should_panic]
fn test_shift_insert_panic_out_of_bounds() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut set: crate::IndexSet<char, TestHasher> = ('a'..='z').collect();
    set.shift_insert(set.len(), 'a');
}

