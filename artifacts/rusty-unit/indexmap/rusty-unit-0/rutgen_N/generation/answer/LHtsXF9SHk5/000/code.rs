// Answer 0

#[derive(Debug)]
struct Bucket<K, V> {
    key: K,
    value: V,
}

struct Iter<'a, K, V> {
    iter: std::slice::Iter<'a, Bucket<K, V>>,
}

impl<'a, K, V> Iter<'a, K, V> {
    pub(super) fn new(entries: &'a [Bucket<K, V>]) -> Self {
        Self {
            iter: entries.iter(),
        }
    }
}

#[test]
fn test_iter_new_with_non_empty_entries() {
    let entries = [
        Bucket { key: 1, value: "A" },
        Bucket { key: 2, value: "B" },
    ];
    let iter = Iter::new(&entries);
    assert_eq!(iter.iter.len(), 2);
}

#[test]
fn test_iter_new_with_empty_entries() {
    let entries: Vec<Bucket<i32, &str>> = Vec::new();
    let iter = Iter::new(&entries);
    assert_eq!(iter.iter.len(), 0);
}

