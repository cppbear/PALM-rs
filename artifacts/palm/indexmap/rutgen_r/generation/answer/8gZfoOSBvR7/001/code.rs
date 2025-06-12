// Answer 0

#[derive(Debug)]
struct Bucket<K, V> {
    key: K,
    value: V,
}

struct MyIter<K, V> {
    iter: std::vec::IntoIter<Bucket<K, V>>,
}

impl<K, V> MyIter<K, V> {
    pub(super) fn new(entries: Vec<Bucket<K, V>>) -> Self {
        Self {
            iter: entries.into_iter(),
        }
    }
}

#[test]
fn test_new_with_non_empty_vec() {
    let buckets = vec![
        Bucket { key: 1, value: "one" },
        Bucket { key: 2, value: "two" },
    ];
    let my_iter = MyIter::new(buckets);
    assert_eq!(my_iter.iter.len(), 2);
}

#[test]
fn test_new_with_empty_vec() {
    let buckets: Vec<Bucket<i32, &str>> = vec![];
    let my_iter = MyIter::new(buckets);
    assert_eq!(my_iter.iter.len(), 0);
}

#[test]
fn test_new_with_single_entry() {
    let buckets = vec![Bucket { key: 3, value: "three" }];
    let my_iter = MyIter::new(buckets);
    assert_eq!(my_iter.iter.len(), 1);
}

