// Answer 0

#[derive(Copy, Clone, Debug)]
struct MyKey {
    value: usize,
}

impl Equivalent<usize> for MyKey {
    fn equivalent(key: &usize) -> bool {
        &self.value == key
    }
}

#[test]
fn test_equivalent_empty_entries() {
    let key = MyKey { value: 1 };
    let entries: Vec<Bucket<usize, usize>> = vec![];
    let func = equivalent(&key, &entries);
    let _result = func(&0); // Out of bounds, should not panic.
}

#[test]
fn test_equivalent_single_entry_non_matching() {
    let key = MyKey { value: 1 };
    let entries = vec![Bucket {
        hash: HashValue::default(),
        key: 2,
        value: 3,
    }];
    let func = equivalent(&key, &entries);
    let _result = func(&0); // Expected to return false.
}

#[test]
fn test_equivalent_single_entry_matching() {
    let key = MyKey { value: 2 };
    let entries = vec![Bucket {
        hash: HashValue::default(),
        key: 2,
        value: 3,
    }];
    let func = equivalent(&key, &entries);
    let _result = func(&0); // Expected to return true.
}

#[test]
fn test_equivalent_multiple_entries_some_matching() {
    let key = MyKey { value: 2 };
    let entries = vec![
        Bucket {
            hash: HashValue::default(),
            key: 3,
            value: 4,
        },
        Bucket {
            hash: HashValue::default(),
            key: 2,
            value: 5,
        },
    ];
    let func = equivalent(&key, &entries);
    let _result1 = func(&0); // Expected to return false.
    let _result2 = func(&1); // Expected to return true.
}

#[test]
fn test_equivalent_large_entries() {
    let key = MyKey { value: 500 };
    let entries: Vec<Bucket<usize, usize>> = (0..1000)
        .map(|i| Bucket {
            hash: HashValue::default(),
            key: i,
            value: i * 2,
        })
        .collect();
    let func = equivalent(&key, &entries);
    let _result = func(&500); // Expected to return true.
}

#[test]
fn test_equivalent_out_of_bounds() {
    let key = MyKey { value: 1 };
    let entries = vec![Bucket {
        hash: HashValue::default(),
        key: 1,
        value: 2,
    }];
    let func = equivalent(&key, &entries);
    let _result = func(&1); // Out of bounds, should not panic.
}

