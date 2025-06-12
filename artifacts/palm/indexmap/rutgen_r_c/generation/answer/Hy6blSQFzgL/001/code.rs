// Answer 0

#[test]
fn test_sorted_by_empty_map() {
    struct TestMap {
        entries: Vec<Bucket<i32, i32>>,
    }
    
    impl TestMap {
        pub fn into_entries(self) -> Vec<Bucket<i32, i32>> {
            self.entries
        }
    }
    
    let map = TestMap { entries: Vec::new() };
    
    let sorted_iter = map.sorted_by(|_k1, _v1, _k2, _v2| Ordering::Equal);
    assert!(sorted_iter.iter.is_empty());
}

#[test]
fn test_sorted_by_single_element() {
    struct TestMap {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl TestMap {
        pub fn into_entries(self) -> Vec<Bucket<i32, i32>> {
            self.entries
        }
    }

    let map = TestMap {
        entries: vec![Bucket { hash: HashValue::default(), key: 1, value: 10 }],
    };

    let mut sorted_iter = map.sorted_by(|_k1, _v1, _k2, _v2| Ordering::Equal);
    let sorted_vec: Vec<_> = sorted_iter.iter.collect();
    
    assert_eq!(sorted_vec.len(), 1);
    assert_eq!(sorted_vec[0].key, 1);
    assert_eq!(sorted_vec[0].value, 10);
}

#[test]
fn test_sorted_by_multiple_elements() {
    struct TestMap {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl TestMap {
        pub fn into_entries(self) -> Vec<Bucket<i32, i32>> {
            self.entries
        }
    }

    let map = TestMap {
        entries: vec![
            Bucket { hash: HashValue::default(), key: 2, value: 20 },
            Bucket { hash: HashValue::default(), key: 1, value: 10 },
            Bucket { hash: HashValue::default(), key: 3, value: 30 },
        ],
    };

    let mut sorted_iter = map.sorted_by(|k1, v1, k2, v2| k1.cmp(k2));
    let sorted_vec: Vec<_> = sorted_iter.iter.collect();
    
    assert_eq!(sorted_vec.len(), 3);
    assert_eq!(sorted_vec[0].key, 1);
    assert_eq!(sorted_vec[1].key, 2);
    assert_eq!(sorted_vec[2].key, 3);
}

#[test]
fn test_sorted_by_stable_sort() {
    struct TestMap {
        entries: Vec<Bucket<char, i32>>,
    }

    impl TestMap {
        pub fn into_entries(self) -> Vec<Bucket<char, i32>> {
            self.entries
        }
    }

    let map = TestMap {
        entries: vec![
            Bucket { hash: HashValue::default(), key: 'b', value: 10 },
            Bucket { hash: HashValue::default(), key: 'a', value: 20 },
            Bucket { hash: HashValue::default(), key: 'b', value: 30 },
            Bucket { hash: HashValue::default(), key: 'a', value: 40 },
        ],
    };

    let mut sorted_iter = map.sorted_by(|k1, v1, k2, v2| {
        let cmp = k1.cmp(k2);
        if cmp == Ordering::Equal {
            v1.cmp(v2)
        } else {
            cmp
        }
    });
    
    let sorted_vec: Vec<_> = sorted_iter.iter.collect();

    assert_eq!(sorted_vec[0].key, 'a');
    assert_eq!(sorted_vec[1].key, 'a');
    assert_eq!(sorted_vec[2].key, 'b');
    assert_eq!(sorted_vec[3].key, 'b');    
}

