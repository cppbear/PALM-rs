// Answer 0

#[test]
fn test_into_iter_new_with_empty_vec() {
    let entries: Vec<Bucket<i32, i32>> = Vec::new();
    let iter: IntoIter<i32, i32> = IntoIter::new(entries);
    assert!(iter.iter.len() == 0);
}

#[test]
fn test_into_iter_new_with_single_entry() {
    let bucket = Bucket { hash: 1, key: 42, value: 100 };
    let entries = vec![bucket];
    let iter: IntoIter<i32, i32> = IntoIter::new(entries);
    assert!(iter.iter.len() == 1);
}

#[test]
fn test_into_iter_new_with_multiple_entries() {
    let bucket1 = Bucket { hash: 1, key: 1, value: 10 };
    let bucket2 = Bucket { hash: 2, key: 2, value: 20 };
    let entries = vec![bucket1, bucket2];
    let iter: IntoIter<i32, i32> = IntoIter::new(entries);
    assert!(iter.iter.len() == 2);
}

