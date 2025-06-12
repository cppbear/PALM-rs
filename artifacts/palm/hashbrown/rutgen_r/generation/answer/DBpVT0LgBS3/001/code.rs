// Answer 0

#[test]
fn test_extract_if_even_numbers() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    for x in 0..10 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    let drained: Vec<i32> = table.extract_if(|&mut v| v % 2 == 0).collect();

    let mut evens = drained.into_iter().collect::<Vec<_>>();
    let mut odds = table.into_iter().collect::<Vec<_>>();
    evens.sort();
    odds.sort();

    assert_eq!(evens, vec![0, 2, 4, 6, 8]);
    assert_eq!(odds, vec![1, 3, 5, 7, 9]);
}

#[test]
fn test_extract_if_no_matches() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    for x in 0..10 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    let drained: Vec<i32> = table.extract_if(|&mut v| v > 10).collect();

    let mut remaining = table.into_iter().collect::<Vec<_>>();
    remaining.sort();

    assert_eq!(drained, Vec::<i32>::new());
    assert_eq!(remaining, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_extract_if_all_matches() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    for x in 0..5 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    let drained: Vec<i32> = table.extract_if(|_| true).collect();

    let remaining = table.into_iter().collect::<Vec<_>>();
    
    assert_eq!(drained, vec![0, 1, 2, 3, 4]);
    assert_eq!(remaining, Vec::<i32>::new());
}

#[test]
fn test_extract_if_mixed() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    for x in 0..15 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    let drained: Vec<i32> = table.extract_if(|&mut v| v % 3 == 0).collect();

    let mut multiples_of_three = drained.into_iter().collect::<Vec<_>>();
    let mut remaining = table.into_iter().collect::<Vec<_>>();
    multiples_of_three.sort();
    remaining.sort();

    assert_eq!(multiples_of_three, vec![0, 3, 6, 9, 12]);
    assert_eq!(remaining, vec![1, 2, 4, 5, 7, 8, 10, 11, 13, 14]);
}

