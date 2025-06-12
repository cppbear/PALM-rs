// Answer 0

#[test]
fn test_as_slice_empty_entries() {
    let mut entries: Vec<Bucket<i32, i32>> = vec![];
    let iter_mut = IterMut2::new(&mut entries);
    let slice = iter_mut.as_slice();
}

#[test]
fn test_as_slice_one_entry() {
    let mut entries: Vec<Bucket<i32, i32>> = vec![Bucket { hash: 0.into(), key: 1, value: 10 }];
    let iter_mut = IterMut2::new(&mut entries);
    let slice = iter_mut.as_slice();
}

#[test]
fn test_as_slice_ten_entries() {
    let mut entries: Vec<Bucket<i32, i32>> = (0..10)
        .map(|i| Bucket { hash: i.into(), key: i + 1, value: i * 10 })
        .collect();
    let iter_mut = IterMut2::new(&mut entries);
    let slice = iter_mut.as_slice();
}

