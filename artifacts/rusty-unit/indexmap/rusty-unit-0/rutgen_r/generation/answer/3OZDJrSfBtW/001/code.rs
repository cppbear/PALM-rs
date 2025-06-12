// Answer 0

#[test]
fn test_new_with_empty_entries() {
    let mut entries: Vec<Bucket<i32, i32>> = Vec::new();
    let iter = new(&mut entries);
    assert!(iter.iter.size_hint().0 == 0); // Check that the iterator has no elements.
}

#[test]
fn test_new_with_single_entry() {
    let mut entries: Vec<Bucket<i32, i32>> = vec![Bucket { key: 1, value: 10 }];
    let iter = new(&mut entries);
    assert_eq!(iter.iter.next(), Some(&mut Bucket { key: 1, value: 10 })); // Check that the iterator yields the single entry.
}

#[test]
fn test_new_with_multiple_entries() {
    let mut entries: Vec<Bucket<i32, i32>> = vec![
        Bucket { key: 1, value: 10 },
        Bucket { key: 2, value: 20 },
        Bucket { key: 3, value: 30 },
    ];
    let mut iter = new(&mut entries);

    let results: Vec<_> = iter.iter.by_ref().map(|bucket| (bucket.key, bucket.value)).collect();
    assert_eq!(results, vec![(1, 10), (2, 20), (3, 30)]); // Check that the iterator returns all multiple entries correctly.
}

#[test]
#[should_panic] // If we expect a mutable borrow after move.
fn test_new_panic_on_borrow_after_move() {
    let mut entries: Vec<Bucket<i32, i32>> = vec![Bucket { key: 1, value: 10 }];
    let iter = new(&mut entries);
    drop(iter); // Make sure to drop iter before trying to access entries again.
    let _ = entries[0]; // This should not panic.
}

