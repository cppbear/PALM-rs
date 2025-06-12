// Answer 0

#[test]
fn test_insert_bulk_no_grow_with_one_entry() {
    let mut indices = hash_table::HashTable::with_capacity(2); // capacity 2
    indices.insert_unique(0, 0, |_| unreachable!()); // pre-fill one entry
    let entries = vec![Bucket {
        hash: HashValue(1),
        key: 1,
        value: "value1",
    }];
    insert_bulk_no_grow(&mut indices, &entries);
}

#[test]
fn test_insert_bulk_no_grow_with_multiple_entries() {
    let mut indices = hash_table::HashTable::with_capacity(10); // capacity 10
    for i in 0..5 {
        indices.insert_unique(i as u64, i, |_| unreachable!()); // pre-fill five entries
    }
    let entries: Vec<Bucket<usize, &str>> = (5..10)
        .map(|i| Bucket {
            hash: HashValue(i as usize),
            key: i,
            value: "value",
        })
        .collect();
    insert_bulk_no_grow(&mut indices, &entries);
}

#[test]
fn test_insert_bulk_no_grow_edge_case() {
    let mut indices = hash_table::HashTable::with_capacity(1000); // capacity 1000
    for i in 0..999 {
        indices.insert_unique(i as u64, i, |_| unreachable!()); // pre-fill 999 entries
    }
    let entries = vec![Bucket {
        hash: HashValue(999),
        key: 999,
        value: "last_value",
    }];
    insert_bulk_no_grow(&mut indices, &entries);
}

#[should_panic]
fn test_insert_bulk_no_grow_exceed_capacity() {
    let mut indices = hash_table::HashTable::with_capacity(5); // capacity 5
    for i in 0..3 {
        indices.insert_unique(i as u64, i, |_| unreachable!()); // pre-fill three entries
    }
    let entries = vec![Bucket {
        hash: HashValue(3),
        key: 3,
        value: "value3",
    }, Bucket {
        hash: HashValue(4),
        key: 4,
        value: "value4",
    }];
    insert_bulk_no_grow(&mut indices, &entries); // should panic
}

#[test]
fn test_insert_bulk_no_grow_varied_key_value() {
    let mut indices = hash_table::HashTable::with_capacity(6); // capacity 6
    for i in 0..4 {
        indices.insert_unique(i as u64, i, |_| unreachable!()); // pre-fill four entries
    }
    let entries = vec![
        Bucket {
            hash: HashValue(4),
            key: 4,
            value: "value4",
        },
        Bucket {
            hash: HashValue(5),
            key: 5,
            value: "value5",
        }
    ];
    insert_bulk_no_grow(&mut indices, &entries); 
}

