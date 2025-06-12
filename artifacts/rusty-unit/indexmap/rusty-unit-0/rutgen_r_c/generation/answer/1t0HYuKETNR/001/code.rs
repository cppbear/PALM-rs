// Answer 0

#[test]
fn test_try_reserve_exact_success() {
    let mut index_set: super::IndexSet<i32, std::collections::hash_map::RandomState> = 
        super::IndexSet::with_capacity_and_hasher(5, std::collections::hash_map::RandomState::new());
    
    let result = index_set.try_reserve_exact(3);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_try_reserve_exact_zero_capacity() {
    let mut index_set: super::IndexSet<i32, std::collections::hash_map::RandomState> = 
        super::IndexSet::with_capacity_and_hasher(5, std::collections::hash_map::RandomState::new());

    let _result = index_set.try_reserve_exact(usize::MAX); // This might panic due to numeric overflow
}

#[test]
fn test_try_reserve_exact_increase_capacity() {
    let mut index_set: super::IndexSet<i32, std::collections::hash_map::RandomState> = 
        super::IndexSet::with_capacity_and_hasher(10, std::collections::hash_map::RandomState::new());
    
    index_set.try_reserve_exact(5).unwrap();

    assert_eq!(index_set.capacity(), 10); // Test might not pass depending on implementation specifics
}

#[test]
fn test_try_reserve_exact_capacity_not_less_than_current() {
    let mut index_set: super::IndexSet<i32, std::collections::hash_map::RandomState> = 
        super::IndexSet::with_capacity_and_hasher(2, std::collections::hash_map::RandomState::new());
    
    index_set.try_reserve_exact(2).unwrap();
    index_set.try_reserve_exact(1).unwrap();

    assert!(index_set.capacity() >= 5); // Expect at least 5 capacity due to reservations
} 

#[test]
fn test_try_reserve_exact_non_zero_additional_check() {
    let mut index_set: super::IndexSet<i32, std::collections::hash_map::RandomState> = 
        super::IndexSet::with_capacity_and_hasher(0, std::collections::hash_map::RandomState::new());

    let result = index_set.try_reserve_exact(5);
    assert!(result.is_ok());
    assert!(index_set.len() == 0);
} 

#[test]
fn test_try_reserve_exact_for_large_value() {
    let mut index_set: super::IndexSet<i32, std::collections::hash_map::RandomState> = 
        super::IndexSet::with_capacity_and_hasher(100, std::collections::hash_map::RandomState::new());

    let result = index_set.try_reserve_exact(1000);
    assert!(result.is_ok());
} 

#[test]
fn test_try_reserve_exact_empty_index_set() {
    let mut index_set: super::IndexSet<i32, std::collections::hash_map::RandomState> = 
        super::IndexSet::with_capacity_and_hasher(0, std::collections::hash_map::RandomState::new());
    
    let result = index_set.try_reserve_exact(10);
    assert!(result.is_ok());
    assert!(index_set.len() == 0);
}     


