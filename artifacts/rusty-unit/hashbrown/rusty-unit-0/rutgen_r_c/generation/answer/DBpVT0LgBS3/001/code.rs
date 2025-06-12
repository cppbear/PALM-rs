// Answer 0

#[test]
fn test_extract_if_with_even_numbers() {
    struct TestAllocator;

    impl Allocator for TestAllocator {}

    let mut table = HashTable::new_in(TestAllocator);
    let hasher = |val: &usize| *val;

    for x in 0..10 {
        table.insert_unique(hasher(&x), x, hasher);
    }

    let drained: Vec<usize> = table.extract_if(|v| *v % 2 == 0).collect();

    let mut evens: Vec<_> = drained.into_iter().collect();
    let mut leftovers: Vec<_> = table.iter().cloned().collect();

    evens.sort();
    leftovers.sort();

    assert_eq!(evens, vec![0, 2, 4, 6, 8]);
    assert_eq!(leftovers, vec![1, 3, 5, 7, 9]);
}

#[test]
fn test_extract_if_with_no_elements() {
    struct TestAllocator;

    impl Allocator for TestAllocator {}

    let mut table = HashTable::new_in(TestAllocator);
    
    let drained: Vec<usize> = table.extract_if(|_| true).collect();

    assert!(drained.is_empty());
}

#[test]
fn test_extract_if_with_all_elements() {
    struct TestAllocator;

    impl Allocator for TestAllocator {}

    let mut table = HashTable::new_in(TestAllocator);
    let hasher = |val: &usize| *val;

    for x in 0..5 {
        table.insert_unique(hasher(&x), x, hasher);
    }

    let drained: Vec<usize> = table.extract_if(|_| true).collect();

    let leftovers: Vec<_> = table.iter().cloned().collect();

    assert_eq!(drained, vec![0, 1, 2, 3, 4]);
    assert!(leftovers.is_empty());
}

#[test]
fn test_extract_if_short_circuit() {
    struct TestAllocator;

    impl Allocator for TestAllocator {}

    let mut table = HashTable::new_in(TestAllocator);
    let hasher = |val: &usize| *val;

    for x in 0..8 {
        table.insert_unique(hasher(&x), x, hasher);
    }

    let mut count = 0;
    let drained: Vec<usize> = table.extract_if(|&mut v| {
        if count < 3 {
            count += 1;
            true
        } else {
            false
        }
    }).collect();

    let leftovers: Vec<_> = table.iter().cloned().collect();

    assert_eq!(drained, vec![0, 1, 2]);
    assert_eq!(leftovers, vec![3, 4, 5, 6, 7]);
}

