// Answer 0

#[test]
fn test_new_with_non_empty_index_sets() {
    use indexmap::indexset::{IndexSet, self};
    use std::collections::hash_map::RandomState;

    let mut set1 = IndexSet::new();
    let mut set2 = IndexSet::new();
    
    set1.insert("apple");
    set1.insert("banana");
    
    set2.insert("banana");
    set2.insert("cherry");

    let result: indexset::Iter<&str, RandomState> = new(&set1, &set2);

    let result_vec: Vec<_> = result.collect();
    let expected_vec = vec!["apple", "banana", "cherry"];
    assert_eq!(result_vec, expected_vec);
}

#[test]
fn test_new_with_empty_set1() {
    use indexmap::indexset::{IndexSet, self};
    use std::collections::hash_map::RandomState;

    let set1: IndexSet<&str> = IndexSet::new();
    let mut set2 = IndexSet::new();
    
    set2.insert("banana");
    set2.insert("cherry");

    let result: indexset::Iter<&str, RandomState> = new(&set1, &set2);

    let result_vec: Vec<_> = result.collect();
    let expected_vec = vec!["banana", "cherry"];
    assert_eq!(result_vec, expected_vec);
}

#[test]
fn test_new_with_empty_sets() {
    use indexmap::indexset::{IndexSet, self};
    use std::collections::hash_map::RandomState;
    
    let set1: IndexSet<&str> = IndexSet::new();
    let set2: IndexSet<&str> = IndexSet::new();

    let result: indexset::Iter<&str, RandomState> = new(&set1, &set2);

    let result_vec: Vec<_> = result.collect();
    let expected_vec: Vec<&str> = Vec::new();
    assert_eq!(result_vec, expected_vec);
}

#[test]
fn test_new_with_identical_sets() {
    use indexmap::indexset::{IndexSet, self};
    use std::collections::hash_map::RandomState;

    let mut set1 = IndexSet::new();
    let mut set2 = IndexSet::new();
    
    set1.insert("apple");
    set1.insert("banana");
    
    set2.insert("apple");
    set2.insert("banana");

    let result: indexset::Iter<&str, RandomState> = new(&set1, &set2);

    let result_vec: Vec<_> = result.collect();
    let expected_vec = vec!["apple", "banana"];
    assert_eq!(result_vec, expected_vec);
}

