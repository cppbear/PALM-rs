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
fn test_new_with_empty_entries() {
    let entries: Vec<Bucket<i32, i32>> = Vec::new();
    let iter = MyIter::new(entries);
    assert_eq!(iter.iter.len(), 0);
}

#[test]
fn test_new_with_single_entry() {
    let entries = vec![Bucket { key: 1, value: 2 }];
    let iter = MyIter::new(entries);
    assert_eq!(iter.iter.len(), 1);
    assert_eq!(iter.iter.next().unwrap().key, 1);
    assert_eq!(iter.iter.next().unwrap().value, 2);
}

#[test]
fn test_new_with_multiple_entries() {
    let entries = vec![
        Bucket { key: 1, value: "a" },
        Bucket { key: 2, value: "b" },
    ];
    let iter = MyIter::new(entries);
    assert_eq!(iter.iter.len(), 2);
    
    let first = iter.iter.next().unwrap();
    assert_eq!(first.key, 1);
    assert_eq!(first.value, "a");
    
    let second = iter.iter.next().unwrap();
    assert_eq!(second.key, 2);
    assert_eq!(second.value, "b");
}

#[test]
fn test_new_with_panic_on_large_vector() {
    let entries: Vec<Bucket<i32, i32>> = (0..1_000_000)
        .map(|i| Bucket { key: i, value: i * 2 })
        .collect();
    let iter = MyIter::new(entries);
    assert_eq!(iter.iter.len(), 1_000_000);
}

