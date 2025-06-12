// Answer 0

#[test]
fn test_extract_if() {
    struct MyAllocator;
    impl Allocator for MyAllocator {}

    let mut table = HashTable::new_in(MyAllocator);
    let hasher = |val: &i32| *val as u64;

    for x in 0..8 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    
    let drained: Vec<i32> = table.extract_if(|v| *v % 2 == 0).collect();

    let evens: Vec<_> = drained;
    let mut odds = table.into_iter().collect::<Vec<_>>();
    odds.sort();
    
    assert_eq!(evens, vec![0, 2, 4, 6]);
    assert_eq!(odds, vec![1, 3, 5, 7]);
}

#[test]
fn test_extract_if_empty_table() {
    struct MyAllocator;
    impl Allocator for MyAllocator {}

    let mut table = HashTable::new_in(MyAllocator);
    let drained: Vec<i32> = table.extract_if(|_| true).collect();
    
    assert!(drained.is_empty());
}

#[test]
fn test_extract_if_no_match() {
    struct MyAllocator;
    impl Allocator for MyAllocator {}

    let mut table = HashTable::new_in(MyAllocator);
    let hasher = |val: &i32| *val as u64;

    for x in 0..8 {
        table.insert_unique(hasher(&x), x, hasher);
    }

    let drained: Vec<i32> = table.extract_if(|_| false).collect();
    
    let remaining: Vec<i32> = table.into_iter().collect();
    
    assert_eq!(drained.len(), 0);
    assert_eq!(remaining, vec![0, 1, 2, 3, 4, 5, 6, 7]);
}

