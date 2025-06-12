// Answer 0

#[derive(Debug)]
struct Bucket<K, V> {
    key: K,
    value: V,
    hash: HashValue,
}

#[derive(Debug)]
struct HashValue(u64);

#[derive(Debug)]
struct Indices {
    capacity: usize,
    len: usize,
}

impl Indices {
    fn new(capacity: usize) -> Self {
        Self { capacity, len: 0 }
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn len(&self) -> usize {
        self.len
    }

    fn insert_unique<F>(&mut self, _hash: &HashValue, _index: usize, _f: F)
    where
        F: FnOnce(),
    {
        if self.len < self.capacity {
            self.len += 1;
        }
    }
}

#[test]
fn test_insert_bulk_no_grow_success() {
    let mut indices = Indices::new(10);
    let entries = vec![
        Bucket { key: 1, value: "a", hash: HashValue(0) },
        Bucket { key: 2, value: "b", hash: HashValue(1) },
        Bucket { key: 3, value: "c", hash: HashValue(2) },
    ];

    insert_bulk_no_grow(&mut indices, &entries);
    assert_eq!(indices.len(), 3);
}

#[test]
#[should_panic]
fn test_insert_bulk_no_grow_not_enough_capacity() {
    let mut indices = Indices::new(2);
    let entries = vec![
        Bucket { key: 1, value: "a", hash: HashValue(0) },
        Bucket { key: 2, value: "b", hash: HashValue(1) },
        Bucket { key: 3, value: "c", hash: HashValue(2) },
    ];

    insert_bulk_no_grow(&mut indices, &entries);
}

