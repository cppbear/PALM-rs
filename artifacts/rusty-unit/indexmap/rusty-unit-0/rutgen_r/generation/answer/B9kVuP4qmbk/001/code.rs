// Answer 0

#[derive(Debug)]
struct Bucket<K, V> {
    key: K,
    value: V,
}

#[derive(Debug)]
struct MockMap<K, V> {
    buckets: Vec<Bucket<K, V>>,
}

impl<K, V> MockMap<K, V> {
    pub(crate) fn into_boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub(crate) fn into_vec(self: Box<Self>) -> Vec<Bucket<K, V>> {
        self.buckets
    }
}

impl<K, V> MockMap<K, V> {
    pub(crate) fn into_entries(self: Box<Self>) -> Vec<Bucket<K, V>> {
        self.into_boxed().into_vec()
    }
}

#[test]
fn test_into_entries_empty() {
    let map = MockMap::<i32, i32> {
        buckets: vec![],
    };
    
    let result = map.into_boxed().into_entries();
    assert_eq!(result, vec![]);
}

#[test]
fn test_into_entries_single() {
    let map = MockMap::<i32, i32> {
        buckets: vec![Bucket { key: 1, value: 2 }],
    };
    
    let result = map.into_boxed().into_entries();
    assert_eq!(result, vec![Bucket { key: 1, value: 2 }]);
}

#[test]
fn test_into_entries_multiple() {
    let map = MockMap::<i32, i32> {
        buckets: vec![
            Bucket { key: 1, value: 2 },
            Bucket { key: 3, value: 4 },
        ],
    };
    
    let result = map.into_boxed().into_entries();
    assert_eq!(result, vec![
        Bucket { key: 1, value: 2 },
        Bucket { key: 3, value: 4 },
    ]);
}

