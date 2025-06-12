// Answer 0

#[test]
fn test_as_slice_non_empty() {
    let buckets = [
        Bucket { hash: 1, key: 'a', value: 10 },
        Bucket { hash: 2, key: 'b', value: 20 },
    ];
    let iter = Iter::new(&buckets);
    let slice = iter.as_slice();
}

#[test]
fn test_as_slice_empty() {
    let buckets: [Bucket<char, i32>; 0] = [];
    let iter = Iter::new(&buckets);
    let slice = iter.as_slice();
}

#[test]
fn test_as_slice_single_element() {
    let buckets = [
        Bucket { hash: 1, key: 'a', value: 10 },
    ];
    let iter = Iter::new(&buckets);
    let slice = iter.as_slice();
}

#[test]
fn test_as_slice_large_slice() {
    let buckets: Vec<Bucket<u32, u32>> = (0..usize::MAX).map(|i| Bucket { hash: i as u64, key: i, value: i * 10 }).collect();
    let iter = Iter::new(&buckets);
    let slice = iter.as_slice();
}

