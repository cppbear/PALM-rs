// Answer 0

#[test]
fn test_iter_hash_mut() {
    use hashbrown::{HashTable, Global};
    use std::hash::BuildHasher;

    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::hash::SipHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::hash::SipHasher::new()
        }
    }

    let mut table = HashTable::new_in(Global);
    let hasher = TestHasher;

    table.insert_unique(hasher.build_hasher().finish(), 2, |val| val.hash(&mut hasher.build_hasher()));
    table.insert_unique(hasher.build_hasher().finish(), 3, |val| val.hash(&mut hasher.build_hasher()));
    table.insert_unique(hasher.build_hasher().finish(), 5, |val| val.hash(&mut hasher.build_hasher()));

    for val in table.iter_hash_mut(hasher.build_hasher().finish()) {
        *val *= 2;
    }

    assert_eq!(table.len(), 3);
    let mut vec: Vec<i32> = Vec::new();

    for val in &table {
        vec.push(*val);
    }

    assert!(vec.contains(&4));
    assert!(vec.contains(&6));
    assert_eq!(table.len(), 3);
}

#[test]
fn test_iter_hash_mut_empty_table() {
    use hashbrown::{HashTable, Global};
    use std::hash::BuildHasher;

    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::hash::SipHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::hash::SipHasher::new()
        }
    }

    let mut table = HashTable::new_in(Global);
    let hasher = TestHasher;

    let mut sum = 0;

    for val in table.iter_hash_mut(hasher.build_hasher().finish()) {
        sum += *val;
    }

    assert_eq!(sum, 0);
}

#[test]
fn test_iter_hash_mut_with_no_matching_hash() {
    use hashbrown::{HashTable, Global};
    use std::hash::BuildHasher;

    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::hash::SipHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::hash::SipHasher::new()
        }
    }

    let mut table = HashTable::new_in(Global);
    let hasher = TestHasher;

    table.insert_unique(hasher.build_hasher().finish(), 2, |val| val.hash(&mut hasher.build_hasher()));

    for _ in table.iter_hash_mut(123456) {} // Using a hash that doesn't match any inserted value

    assert_eq!(table.len(), 1);
    assert_eq!(*table.get(hasher.build_hasher().finish(), |&&val| val == 2).unwrap(), 2);
}

