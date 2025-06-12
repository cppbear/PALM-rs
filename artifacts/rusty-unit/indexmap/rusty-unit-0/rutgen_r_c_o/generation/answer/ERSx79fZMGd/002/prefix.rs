// Answer 0

#[test]
fn test_insert_bulk_no_grow_with_min_capacity() {
    let mut indices = hash_table::HashTable::with_capacity(1);
    indices.insert_unique(0, 0, |_| unreachable!());
    
    let entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "One" },
    ];
    
    insert_bulk_no_grow(&mut indices, &entries);
}

#[test]
fn test_insert_bulk_no_grow_with_exact_capacity() {
    let mut indices = hash_table::HashTable::with_capacity(2);
    indices.insert_unique(0, 0, |_| unreachable!());
    
    let entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "One" },
        Bucket { hash: HashValue(2), key: 2, value: "Two" },
    ];
    
    insert_bulk_no_grow(&mut indices, &entries);
}

#[test]
fn test_insert_bulk_no_grow_at_max_capacity() {
    let mut indices = hash_table::HashTable::with_capacity(100);
    for i in 0..50 {
        indices.insert_unique(i, i, |_| unreachable!());
    }

    let entries = (50..100).map(|i| {
        Bucket { hash: HashValue(i as usize), key: i as usize, value: "Value" }
    }).collect::<Vec<_>>();
    
    insert_bulk_no_grow(&mut indices, &entries);
}

#[test]
fn test_insert_bulk_no_grow_multiple_buckets() {
    let mut indices = hash_table::HashTable::with_capacity(20);
    for i in 0..10 {
        indices.insert_unique(i, i, |_| unreachable!());
    }

    let entries = vec![
        Bucket { hash: HashValue(10), key: 10, value: "Ten" },
        Bucket { hash: HashValue(11), key: 11, value: "Eleven" },
    ];

    insert_bulk_no_grow(&mut indices, &entries);
}

#[test]
#[should_panic]
fn test_insert_bulk_no_grow_insufficient_capacity() {
    let mut indices = hash_table::HashTable::with_capacity(3);
    for i in 0..3 {
        indices.insert_unique(i, i, |_| unreachable!());
    }

    let entries = vec![
        Bucket { hash: HashValue(3), key: 3, value: "Three" },
        Bucket { hash: HashValue(4), key: 4, value: "Four" },
    ];

    insert_bulk_no_grow(&mut indices, &entries);
}

#[test]
fn test_insert_bulk_no_grow_large_entry_sets() {
    let mut indices = hash_table::HashTable::with_capacity(50);
    for i in 0..25 {
        indices.insert_unique(i, i, |_| unreachable!());
    }

    let entries = (25..50).map(|i| {
        Bucket { hash: HashValue(i as usize), key: i as usize, value: "Value" }
    }).collect::<Vec<_>>();

    insert_bulk_no_grow(&mut indices, &entries);
}

