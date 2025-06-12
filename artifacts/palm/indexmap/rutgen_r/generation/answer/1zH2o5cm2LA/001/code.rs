// Answer 0

#[derive(Debug)]
struct Bucket<K, V> {
    key: K,
    value: V,
}

struct IteratorWrapper<K, V> {
    iter: std::vec::IntoIter<Bucket<K, V>>,
}

impl<K, V> IteratorWrapper<K, V> {
    pub(super) fn new(entries: Vec<Bucket<K, V>>) -> Self {
        Self {
            iter: entries.into_iter(),
        }
    }
}

#[test]
fn test_new_empty_entries() {
    let entries: Vec<Bucket<i32, i32>> = Vec::new();
    let wrapper = IteratorWrapper::new(entries);
    assert!(wrapper.iter.len() == 0);
}

#[test]
fn test_new_single_entry() {
    let entries = vec![Bucket { key: 1, value: 10 }];
    let wrapper = IteratorWrapper::new(entries);
    assert_eq!(wrapper.iter.len(), 1);
    assert_eq!(wrapper.iter.next().unwrap().key, 1);
    assert_eq!(wrapper.iter.next().unwrap().value, 10);
}

#[test]
fn test_new_multiple_entries() {
    let entries = vec![
        Bucket { key: 1, value: 10 },
        Bucket { key: 2, value: 20 },
        Bucket { key: 3, value: 30 },
    ];
    let wrapper = IteratorWrapper::new(entries);
    assert_eq!(wrapper.iter.len(), 3);
    assert_eq!(wrapper.iter.next().unwrap().key, 1);
    assert_eq!(wrapper.iter.next().unwrap().value, 20);
}

#[test]
fn test_new_entries_panic_condition() {
    let entries = vec![
        Bucket { key: 1, value: 10 },
        Bucket { key: 2, value: 20 },
    ];
    let wrapper = IteratorWrapper::new(entries);
    assert_eq!(wrapper.iter.len(), 2);
    assert!(wrapper.iter.next().is_some());
    assert!(wrapper.iter.next().is_some());
    assert!(wrapper.iter.next().is_none());
}

